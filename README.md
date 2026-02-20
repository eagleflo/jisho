# Jisho (辞書)

[![crates.io](https://img.shields.io/crates/v/jisho.svg)](https://crates.io/crates/jisho)
[![dependency status](https://deps.rs/repo/github/eagleflo/jisho/status.svg)](https://deps.rs/repo/github/eagleflo/jisho)

Jisho is a CLI tool & Rust library that provides offline access to [JMdict][],
the excellent machine-readable Japanese-English dictionary project originally
started by Jim Breen.

## Use

Interactively:

```text
$ jisho
> 積ん読
積ん読【つんどく】- buying books and not reading them, stockpiling books, tsundoku; books bought but not read
> International Space Station
国際宇宙ステーション【こくさいうちゅうステーション】- International Space Station
```

Looking up individual entries:

```text
$ jisho 辞書
辞書【じしょ】- dictionary

$ jisho "quantum mechanics"
量子力学【りょうしりきがく】- quantum mechanics
```

Prefix and postfix matches:

```text
> ＊飛行機
飛行機【ひこうき】- airplane, aeroplane, plane, aircraft
模型飛行機【もけいひこうき】- model plane
成層圏飛行機【せいそうけんひこうき】- stratoplane (hypothetical type of airplane)
無人飛行機【むじんひこうき】- unmanned aircraft, pilotless plane, robot plane, drone
全翼飛行機【ぜんよくひこうき】- flying wing
単葉飛行機【たんようひこうき】- monoplane
ラジコン飛行機【ラジコンひこうき】- radio-controlled aircraft, RC airplane
紙飛行機【かみひこうき】- paper plane, paper airplane, paper aeroplane
無尾翼飛行機【むびよくひこうき】- tailless airplane, tail-less airplane
軽飛行機【けいひこうき】- light aircraft
水上飛行機【すいじょうひこうき】- hydroplane, seaplane

> 飛行機＊
飛行機【ひこうき】- airplane, aeroplane, plane, aircraft
飛行機雲【ひこうきぐも】- contrail, vapor trail, vapour trail
飛行機恐怖症【ひこうききょうふしょう】- fear of flying, aviophobia, aerophobia
飛行機酔い【ひこうきよい】- airsickness
```

Wildcard matches:

```
> 飛？機
飛行機【ひこうき】- airplane, aeroplane, plane, aircraft
```

[JMdict]: http://www.edrdg.org/wiki/index.php/JMdict-EDICT_Dictionary_Project
