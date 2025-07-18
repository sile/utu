# utu

**WIP**

TUI Text editor for pixel arts.

ASCII canvas
------------

```
+--------------------------------+
|                                |
|                                |
|                                |
|                                |
|  oooooooooo                    |
|  o        o                    |
|  o        o                    |
|  oooooooooo                    |
|                                |
|        ooooooooooooo           |
|        o           o           |
|        o           o           |
|        o           o           |
|        ooooooooooooo           |
|                                |
|                                |
+--------------------------------+
```

Multi-byte char canvas
----------------------

Using Unicode block characters for pixel-perfect alignment:

```
|█░█░█░█░█░█░█░█░|
|░█░█░█░█░█░█░█░█|
|█░█░█░█░█░█░█░█░|
|░█░█░█░█░█░█░█░█|
|█░█░█░█░█░█░█░█░|
|░█░█░█░█░█░█░█░█|
|█░█░█░█░█░█░█░█░|
|░█░█░█░█░█░█░█░█|
|█░█░█░█░█░█░█░█░|
|░█░█░█░█░█░█░█░█|
|█░█░█░█░█░█░█░█░|
|░█░█░█░█░█░█░█░█|
|█░█░█░█░█░█░█░█░|
|░█░█░█░█░█░█░█░█|
|█░█░█░█░█░█░█░█░|
|░█░█░█░█░█░█░█░█|
```

Character reference:
- `█` (U+2588) - Full block (black)
- `░` (U+2591) - Light shade (light gray/white)
- `▓` (U+2593) - Dark shade 
- `▒` (U+2592) - Medium shade


Emoji canvas
------------

Using colored emoji squares for vibrant pixel art:

```
+--------------------------------+
|⬛🟨⬛🟨⬛🟨⬛🟨⬛🟨⬛🟨⬛🟨⬛🟨|
|🟨⬛🟨⬛🟨⬛🟨⬛🟨⬛🟨⬛🟨⬛🟨⬛|
|⬛🟨⬛🟨⬛🟨⬛🟨⬛🟨⬛🟨⬛🟨⬛🟨|
|🟨⬛🟨⬛🟨⬛🟨⬛🟨⬛🟨⬛🟨⬛🟨⬛|
|⬛🟨⬛🟨⬛🟨⬛🟨⬛🟨⬛🟨⬛🟨⬛🟨|
|🟨⬛🟨⬛🟨⬛🟨⬛🟨⬛🟨⬛🟨⬛🟨⬛|
|⬛🟨⬛🟨⬛🟨⬛🟨⬛🟨⬛🟨⬛🟨⬛🟨|
|🟨⬛🟨⬛🟨⬛🟨⬛🟨⬛🟨⬛🟨⬛🟨⬛|
|⬛🟨⬛🟨⬛🟨⬛🟨⬛🟨⬛🟨⬛🟨⬛🟨|
|🟨⬛🟨⬛🟨⬛🟨⬛🟨⬛🟨⬛🟨⬛🟨⬛|
|⬛🟨⬛🟨⬛🟨⬛🟨⬛🟨⬛🟨⬛🟨⬛🟨|
|🟨⬛🟨⬛🟨⬛🟨⬛🟨⬛🟨⬛🟨⬛🟨⬛|
|⬛🟨⬛🟨⬛🟨⬛🟨⬛🟨⬛🟨⬛🟨⬛🟨|
|🟨⬛🟨⬛🟨⬛🟨⬛🟨⬛🟨⬛🟨⬛🟨⬛|
|⬛🟨⬛🟨⬛🟨⬛🟨⬛🟨⬛🟨⬛🟨⬛🟨|
|🟨⬛🟨⬛🟨⬛🟨⬛🟨⬛🟨⬛🟨⬛🟨⬛|
+--------------------------------+
```

Character reference:
- `⬜` (U+2B1C) - White large square
- `🟨` (U+1F7E8) - Yellow square
- `🟧` (U+1F7E7) - Orange square
- `🟫` (U+1F7EB) - Brown square
- `⬛` (U+2B1B) - Black large square
- `🟥` (U+1F7E5) - Red square
- `🟩` (U+1F7E9) - Green square
- `🟦` (U+1F7E6) - Blue square
- `🟪` (U+1F7EA) - Purple square

4 colors:
- `⬜` (U+2B1C) - White large square
- `🟨` (U+1F7E8) - Yellow square
- `🟧` (U+1F7E7) - Orange square
- `🟥` (U+1F7E5) - Red square

```
⬜⬜🟥🟥🟥🟥⬜⬜
⬜🟥🟥🟧🟧🟥🟥⬜
🟥🟥🟧🟨🟨🟧🟥🟥
🟥🟧🟧🟨🟨🟧🟧🟥
🟥🟧🟨🟨🟨🟨🟧🟥
🟧🟧🟨🟨🟨🟨🟧🟧
🟧🟨🟨🟨🟨🟨🟨🟧
🟨🟨🟨🟨🟨🟨🟨🟨
```


Another pattern:
- `⬜` (U+2B1C) - White large square
- `🟨` (U+1F7E8) - Yellow square
- `🟫` (U+1F7EB) - Brown square
- `⬛` (U+2B1B) - Black large square
