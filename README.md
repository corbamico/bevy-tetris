# bevy-tetris

bevy-tetris clone tetris game using rust/bevy

## Some Notes in Coding

* Keyboard::Up is roration
* Game Board as 10x20
* Each Dot is drawed as NodeComponents 20px\*20px, with Child 16px\*16px (and with Child 12px\*12px)
* Board Dot(0,0) as Pixel location (13px,13px) as code in consts.rs
* bricks type as : I,J,L,Z,S,T,O as code in consts.rs
* rotation system use as simple as Nintendo [here](https://tetris.fandom.com/wiki/Nintendo_Rotation_System)
* tetris speeding use delay = 725 * .85 ^ level + level from [dwhacks](http://gist.github.com/dwhacks/8644250), refer to src/speeds.rs
* tetris scoring use [Original Nintendo Scoring System](https://tetris.fandom.com/wiki/Scoring), refer to src/speeds.rs

## Snapshoot

[!screen](./docs/screen.png)

## Thanks

inspired by [flappy_bevy](https://github.com/TanTanDev/flappy_bevy) and [bevy-snake](https://mbuffett.com/posts/bevy-snake-tutorial/)

## License

* GPLv3, Copyright by corbamico@163.com
* Assets digital7mono.ttf: True Type Fonts: DIGITAL-7 version 1.02 (by Sizenko Alexander,Style-7)
