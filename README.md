# xtc-rs
Excel to csv in Rust


### `git tag` dance

```sh
git tag --delete v0.1.0
git push --delete origin v0.1.0
git add .
git commit -m "commit msg"
git push
git tag -a v0.1.0 -m "v0.1.0"
git push origin --tags
```

Only the presence of a `git tag` will trigger a Github Actions build