import { Octokit } from '@octokit/rest'
import * as path from '@std/path'
import * as toml from '@std/toml'

interface Manifest {
  package: {
    version: string,
    name: string
  }
}

interface CrateInfo {
  crate: {
    max_version: string,
    newest_version: string,
    max_stable_version: string
  }
}

// class SemanticVersion {
//   public readonly major: string
//   public readonly minor: string
//   public readonly patch: string
//   public readonly preReleaseTag: string | null

//   constructor(version: string) {
//     const [semVer, preReleaseTag] = version.split("-")
//     const [major, minor, patch] = semVer.split("`.")
//     this.major = major
//     this.minor = minor
//     this.patch = patch
//     this.preReleaseTag = preReleaseTag ? preReleaseTag : null
//   }
// }

async function main() {
  const GITHUB_TOKEN = Deno.env.get("GITHUB_TOKEN")
  const GITHUB_REPOSITORY = Deno.env.get("GITHUB_REPOSITORY")

  if (!GITHUB_TOKEN || !GITHUB_REPOSITORY) {
    throw new Error("can not find necessary env")
  }

  const [OWNER, REPO_NAME] = GITHUB_REPOSITORY.split("/")

  if (!REPO_NAME || !OWNER) {
    throw new Error("GITHUB_REPOSITORY does not meet the format")
  }

  const octokit = new Octokit({
    auth: GITHUB_TOKEN
  })

  const baseDir = Deno.cwd()

  const manifestPath = path.join(baseDir, "Cargo.toml")
  const manifest = toml.parse(await Deno.readTextFile(manifestPath)) as unknown as Manifest

  const reqUrl = `https://crates.io/api/v1/crates/${manifest.package.name}`
  const crateInfo = await (await fetch(reqUrl)).json() as CrateInfo

  if (crateInfo.crate.newest_version === manifest.package.version) {
    return
  }

  const tag = `v${manifest.package.version}`

  await octokit.repos.createRelease({
    repo: REPO_NAME,
    owner: OWNER,
    tag_name: tag,
    name: tag,
    generate_release_notes: true
  })

}

await main()

