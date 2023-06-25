# Jisho (辞書)

Jisho is a CLI tool & Rust library that provides offline access to [JMdict][],
the excellent machine-readable Japanese-English dictionary project originally
started by Jim Breen.

## Use

Interactively:

``` shell
$ jisho
> 積ん読
積ん読【つんどく】- buying books and not reading them, stockpiling books, tsundoku, books bought but not read
> International Space Station
国際宇宙ステーション【こくさいうちゅうステーション】- International Space Station
```

Looking up individual entries:

```shell
$ jisho 辞書
辞書【じしょ】- dictionary
```

```shell
$ jisho "quantum mechanics"
量子力学【りょうしりきがく】- quantum mechanics
```

[JMdict]: http://www.edrdg.org/wiki/index.php/JMdict-EDICT_Dictionary_Project
