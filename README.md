# puzzle-a-day
Solver for the [puzzle-a-day puzzle](https://www.dragonfjord.com/product/a-puzzle-a-day/)

<p align="center">
  <img src="https://i0.wp.com/www.dragonfjord.com/wp-content/uploads/2019/11/IMG_5226-1.jpg?resize=768%2C576&ssl=1" width="350" title="Photo of the physical puzzle">
</p>


## Usage 
```
cargo run --release <day 1-31> <month 1-12>
```

For example, running 
```
cargo run --release 21 5
```
outputs:

```
--- Solution 1 ---
🟪🟪🟪🟧⬛🟧⬛
🟪⬜🟫🟧🟧🟧⬛
🟪⬜🟫🟫🟫🟫🟦
⬜⬜🟨🟨🟦🟦🟦
⬜🟨🟨🟨🟦🟩⬛
🟥🟥🟥🟩🟩🟩🟩
🟥🟥🟥⬛⬛⬛⬛

# and 35 more solutions...
```
