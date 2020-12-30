# case_oscillating_tee
A TEe-cOpYcAt tHAt rAndoMLy ChaNgEs EaCh cHarACtEr'S CAse

## building
```
$ cargo install --path .
```
you may add `--root [INSTALL_HERE]` option with the actual path you wish to
install the binary to, instead of default `~/.cargo/bin`.
please read `cargo help install` before proceeding, tho.

## usage
```
$ case_oscillating_tee > willy_wonka.txt
good morning star shine, the earth says hello!
^D
$ cat willy_wonka.txt
GoOd MOrNiNg sTar sHiNE, THe eArtH sAYs HeLlO!
```
(fyi: i got the idea from [this video](https://youtu.be/webDhBtBchY?t=64))

## license
mit
