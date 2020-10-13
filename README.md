# xtc-rs
**(x)lsx (t)o (c)sv** &rarr; convert xlsx file to a csv file using Rust


### [Standing on the Shoulders](https://en.wikipedia.org/wiki/Standing_on_the_shoulders_of_giants)
Code is simply a wrapper around the [calamine](https://github.com/tafia/calamine) library.  In fact, initial implementation copies their [xlsx to csv example](https://github.com/tafia/calamine/blob/master/examples/excel_to_csv.rs).

In addition, copied methodology within [xlsx-to-csv.rs](https://github.com/zitsen/xlsx2csv.rs/blob/master/src/main.rs) for writing to `csv`.


### Github Actions
Only the presence of a `git tag` will trigger a Github Actions build

Below is the rought `git tag` dance

```sh
# delete local tag
git tag --delete v0.1.0

# delete remote tag
git push --delete origin v0.1.0

# add and commit local changes
git add .
git commit -m "commit msg"

# push local changes to remote
git push

# add local tag
git tag -a v0.1.0 -m "v0.1.0"

# push local tag to remote
git push origin --tags
```
