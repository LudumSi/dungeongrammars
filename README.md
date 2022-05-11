# Dungeon Grammars
## A grid grammar based dungeon generator

## Rule syntax
### Options
All rules start with a line showing their dimensions in Columns,Rows format
Rules which should have rotations generated should have an "R" option
Rules which should be mirrored along the vertical axis should have a "V"
Likewise, rules which should be mirrored along the horizontal axis should have a "H"

Rotation and mirroring is culmulative. So if rotations with mirroring the rotations will also be mirrored. Note that this will affect rule weighting as the generator considers each rotation and mirror to be its own rule.

Example:
3,3,R produces a rule which matches with a 3x3 space with rotations
2,3 produces a rule which matches with a 2x3 space without rotations
2,3,V,R produces a rule which matches with a 2x3 space with rotations. The original and all the rotations are also mirrored in the vertical axis.

### Patterns and Results
The first grid described is the target which will be matched
Grids are separated by lines with equals signs
Further grids are possible results which will replace the target grid 
Result grids are picked randomly with equal weight

The following rule would replace a length of 3 horizontal corridors with a four way intersection.
```
3,3,R
...
ccc
...
=
.C.
c5c
.C.
```

### Tile Descriptions
The following characters in the rules describe the following tile types:

#### Basic Tiles
c: Horizontal corridor
C: Vertical corridor
r: Room
R: Room2 - renders the same as room but rules consider them distinct
~: "Water" tile
.: Empty space

#### Intersection Tiles
These are based on the numpad
```
789             ╔╦╗
456 --maps-to-> ╠╬╣
123             ╚╩╝
```

#### Wildcard Tiles
The ? character has different behaviour depending on whether its in the target or the result.
If the character is in the target, the rule ignores the tile type in that tile. It acts as a wildcard.
If the character is in the result, the existing tile is not replaced.