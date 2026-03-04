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


async function runScript(args: string[], cwd?: string) {
  const cmd = new Deno.Command("/bin/bash", {
    args,
    cwd,
    stdout: "piped",
    stderr: "piped"
  })

  const child = cmd.spawn()
  child.stdout.pipeTo(Deno.stdout.writable)
  child.stderr.pipeTo(Deno.stderr.writable)
  const status = await child.status
  if (!status.success) {
    Deno.exit(status.code)
  }
}

async function main() {
  const GITHUB_TOKEN = Deno.env.get("GITHUB_TOKEN")
  const GITHUB_REPOSITORY = Deno.env.get("GITHUB_REPOSITORY")
  const CARGO_PUBLISH = Deno.env.get("CARGO_PUBLISH")

  if (!GITHUB_TOKEN || !GITHUB_REPOSITORY || !CARGO_PUBLISH) {
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

  // const CARGO_PATH = `${Deno.env.get("HOME")}/.cargo/bin/cargo`

  await runScript(["cargo", "publish", "--token", CARGO_PUBLISH], baseDir)

}

await main()

