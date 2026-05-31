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


async function runScript(command: string, args: string[], cwd?: string) {
  const cmd = new Deno.Command(command, {
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

async function readManifest(baseDir: string, relativePath: string): Promise<Manifest> {
  const manifestPath = path.join(baseDir, relativePath)
  const content = await Deno.readTextFile(manifestPath)
  return toml.parse(content) as unknown as Manifest
}

async function checkCrateVersionExists(name: string, version: string): Promise<boolean> {
  try {
    const reqUrl = `https://crates.io/api/v1/crates/${name}`
    const response = await fetch(reqUrl)
    if (!response.ok) {
      return false
    }
    const crateInfo = await response.json() as CrateInfo
    return crateInfo.crate.newest_version === version
  } catch (_e) {
    return false
  }
}

async function waitForCrateIndex(name: string, version: string, maxWaitMs = 300000, intervalMs = 10000) {
  const startTime = Date.now()
  while (Date.now() - startTime < maxWaitMs) {
    if (await checkCrateVersionExists(name, version)) {
      console.log(`crate ${name}@${version} is now available on crates.io`)
      return
    }
    console.log(`waiting for ${name}@${version} to be available on crates.io...`)
    await new Promise(resolve => setTimeout(resolve, intervalMs))
  }
  throw new Error(`timeout waiting for ${name}@${version} to be available on crates.io`)
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

  const mainManifest = await readManifest(baseDir, "Cargo.toml")
  const mainName = mainManifest.package.name
  const mainVersion = mainManifest.package.version

  if (await checkCrateVersionExists(mainName, mainVersion)) {
    console.log(`${mainName}@${mainVersion} already exists on crates.io, skipping.`)
    return
  }

  const macrosManifest = await readManifest(baseDir, "macros/Cargo.toml")
  const macrosName = macrosManifest.package.name
  const macrosVersion = macrosManifest.package.version

  if (!await checkCrateVersionExists(macrosName, macrosVersion)) {
    console.log(`publishing ${macrosName}@${macrosVersion}...`)
    await runScript("cargo", ["publish", "--token", CARGO_PUBLISH], path.join(baseDir, "macros"))
    console.log(`waiting for ${macrosName}@${macrosVersion} to be indexed...`)
    await waitForCrateIndex(macrosName, macrosVersion)
  } else {
    console.log(`${macrosName}@${macrosVersion} already exists on crates.io, skipping.`)
  }

  const tag = `v${mainVersion}`

  await octokit.repos.createRelease({
    repo: REPO_NAME,
    owner: OWNER,
    tag_name: tag,
    name: tag,
    generate_release_notes: true
  })

  // const CARGO_PATH = `${Deno.env.get("HOME")}/.cargo/bin/cargo`

  await runScript("cargo", ["publish", "--token", CARGO_PUBLISH], baseDir)

}

await main()
