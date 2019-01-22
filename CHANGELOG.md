## 2019-01-22, Version 0.3.0
### Commits
- [[`cb87621fac`](https://github.com/rust-clique/man/commit/cb87621fac17ebfd661ce69b1d64527375227e14)] (cargo-release) version 0.3.0 (Yoshua Wuyts)
- [[`af9a6dfc93`](https://github.com/rust-clique/man/commit/af9a6dfc93c3ea84b45e0b6a3651e3a725fbcea2)] Merge pull request #28 from codesections/example-section (Dylan DPC)
- [[`d104a4a5c3`](https://github.com/rust-clique/man/commit/d104a4a5c337bbe0fc1f030e3db75543ac482b25)] Remove extraneous comment (Daniel Sockwell)
- [[`552ded4f8e`](https://github.com/rust-clique/man/commit/552ded4f8ea794f331502940583dfd393ba81ef0)] Derive default to make Clippy happy (Daniel Sockwell)
- [[`5b7c4a86e5`](https://github.com/rust-clique/man/commit/5b7c4a86e543b5a134a9919cee8f9c9dd1e00e2f)] Add API for example section and update docs (Daniel Sockwell)
- [[`2847380514`](https://github.com/rust-clique/man/commit/28473805145c2dc9ff3d61246755b0f65c48564f)] Update changelog (Yoshua Wuyts)

### Stats
```diff
 CHANGELOG.md     | 25 +++++++++++++++++++++++++
 Cargo.toml       |  2 +-
 README.md        | 12 ++++++++++++
 examples/main.rs | 12 ++++++++++++
 src/example.rs   | 39 +++++++++++++++++++++++++++++++++++++++
 src/lib.rs       |  2 ++
 src/man.rs       | 54 ++++++++++++++++++++++++++++++++++++++++++++++++++++++
 src/prelude.rs   |  1 +
 8 files changed, 146 insertions(+), 1 deletion(-)
```


## 2019-01-18, Version 0.2.0
### Commits
- [[`be9f474a8d`](https://github.com/rust-clique/man/commit/be9f474a8d468f1ee5dcbcb265b1622f9d7b4b07)] (cargo-release) version 0.2.0 (Yoshua Wuyts)
- [[`ebea68ae03`](https://github.com/rust-clique/man/commit/ebea68ae036d85e675cfed68e0f81ea3c8f61048)] Add custom section (#24) (Daniel Sockwell)
- [[`dd0ae08a4d`](https://github.com/rust-clique/man/commit/dd0ae08a4d7a3e5fbe6a5ba6c7921bd4243b8322)] Merge pull request #23 from codesections/README-example (Dylan DPC)
- [[`5f703ae61a`](https://github.com/rust-clique/man/commit/5f703ae61a54881420a56428fb4469e3806e3042)] fmt (dylan_DPC)
- [[`f876dd271d`](https://github.com/rust-clique/man/commit/f876dd271d888c3883ed4f86ed962ca9dd223726)] Clarify installation instructions (Daniel Sockwell)
- [[`3c4d7c27a1`](https://github.com/rust-clique/man/commit/3c4d7c27a1e090160f11ec908a1108d7949b0569)] Fix usage example (Daniel Sockwell)
- [[`48882c80fa`](https://github.com/rust-clique/man/commit/48882c80fa711c2a561aa93486bbe243a1262673)] Update changelog (Yoshua Wuyts)

### Stats
```diff
 CHANGELOG.md     | 34 ++++++++++++++++++++++++++++++++++
 Cargo.toml       |  2 +-
 README.md        | 48 +++++++++++++++++++++++++++++++++++-------------
 examples/main.rs | 17 +++++++++++++----
 src/lib.rs       |  2 ++
 src/man.rs       | 34 ++++++++++++++++++++++++++++++++++
 src/mod.rs       |  9 ++++-----
 src/prelude.rs   |  1 +
 src/section.rs   | 20 ++++++++++++++++++++
 9 files changed, 144 insertions(+), 23 deletions(-)
```


## 2018-11-07, Version 0.1.1
### Commits
- [[`163c3a0cbe`](https://github.com/rust-clique/man/commit/163c3a0cbe54af4e8431ed1ae72cc413d5dbf5ed)] (cargo-release) version 0.1.1 (Yoshua Wuyts)
- [[`892ac22c36`](https://github.com/rust-clique/man/commit/892ac22c36b07b6731464a4ef104c44bcc623c44)] Merge pull request #20 from killercup/ci (Dylan DPC)
- [[`e9466baf2b`](https://github.com/rust-clique/man/commit/e9466baf2b125b4a90818b92a301d45ea6cfe489)] clippy (Pascal Hertleif)
- [[`47d317f0ac`](https://github.com/rust-clique/man/commit/47d317f0acf2d4cdc44875833e27fc3ed19b3e69)] rustfmt (Pascal Hertleif)
- [[`33a01f330e`](https://github.com/rust-clique/man/commit/33a01f330ed80d1f6cc4fb4d8d2c5def74b2b1ce)] Update travis settings (Pascal Hertleif)
- [[`cd8f1d5d88`](https://github.com/rust-clique/man/commit/cd8f1d5d88ba1522af7f7c9839b2e9a76b1a0efa)] Merge pull request #13 from rust-clique/docs (Pascal Hertleif)
- [[`4c848d3b65`](https://github.com/rust-clique/man/commit/4c848d3b65f18b4edd01b92a8106cc6c2768ebdf)] Merge pull request #19 from Kixunil/section-description (Pascal Hertleif)
- [[`542fe19b31`](https://github.com/rust-clique/man/commit/542fe19b31334f9c7827d81923d3d51db92c3ae9)] Added description section. (Martin Habovstiak)
- [[`904b4c4193`](https://github.com/rust-clique/man/commit/904b4c4193bf70d7641b14f5f642fed26612fe2c)] Use correct name (#16) (Jan-Erik Rediger)
- [[`2e64a4e637`](https://github.com/rust-clique/man/commit/2e64a4e6370f83a40d17fc96be5db1fade747a98)] Accept everything that can be turned into a string (#15) (Jan-Erik Rediger)
- [[`3ee6dbbd3a`](https://github.com/rust-clique/man/commit/3ee6dbbd3ac5b624a49413c77e7be2ae1f8b2a96)] document more (Yoshua Wuyts)
- [[`ede4e240c9`](https://github.com/rust-clique/man/commit/ede4e240c93b2ba22b39e6bbc835710df91b13f5)] Merge pull request #12 from rust-clique/example (Dylan DPC)
- [[`16d8709b14`](https://github.com/rust-clique/man/commit/16d8709b1404d3443bb6a9d179629a1bcafb1d3b)] add example output (Yoshua Wuyts)
- [[`53a03672c3`](https://github.com/rust-clique/man/commit/53a03672c3a256c550c070aa634c0d41a665d052)] (cargo-release) start next development iteration 0.1.1-alpha.0 (Yoshua Wuyts)

### Stats
```diff
 .travis.yml        | 17 +++++++----------
 Cargo.toml         |  2 +-
 README.md          | 36 ++++++++++++++++++++++++++++++++++--
 examples/main.rs   | 14 +++++---------
 src/arg.rs         |  1 +
 src/author.rs      |  2 +-
 src/environment.rs |  2 +-
 src/flag.rs        | 10 +++-------
 src/man.rs         | 43 ++++++++++++++++++++++++++++++++++---------
 src/option.rs      |  2 +-
 src/prelude.rs     |  2 +-
 11 files changed, 89 insertions(+), 42 deletions(-)
```


