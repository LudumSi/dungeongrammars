# Dungeon Grammars
## A grid grammar based dungeon generator

This program works by using simple find and replace rules to add complexity to a basic form. This base dungeon is located at base.txt and is customizable using the same syntax as the rules, as explained below.

## Rule syntax
### Options
All rules start with a line showing their dimensions in Row,Column format

The dimensions are optionally followed by a list of comma-separated flags. The base map does not accept flags.
Rules which should have rotations generated should have an "R" option
Rules which should be mirrored along the vertical axis should have a "V"
Likewise, rules which should be mirrored along the horizontal axis should have a "H"

Rotation and mirroring is culmulative. So if rotations with mirroring the rotations will also be mirrored. Note that this will affect rule weighting as the generator considers each rotation and mirror to be its own rule.

An equals sign immediately follows the header on its own line. 

The equals sign is optionally followed by an integer representing the random weight of the rule. By default, weight is 1. Putting a weight under the header (above the rule's target) affects the weighting of searching for that target among all rules in the folder. Putting a weight above a result affects the weighting of which result is picked (if there are multiple results).

Example headers:
```
3,3
R
=
``` 
This produces a rule which matches with a 3x3 space with rotations.


```
2,3
=5 
``` 
This produces a rule which matches with a 2x3 space without rotations and is 5 times more likely to be picked than a rule with a weight of 1.


```
2,3
R,V
=
``` 
This produces a rule which matches with a 2x3 space with rotations. The original and all the rotations are also mirrored in the vertical axis.

### Patterns and Results
The first grid described is the target which will be matched
Grids are separated by lines with equals signs
Further grids are possible results which will replace the target grid 
Result grids are picked randomly with equal weight

The following rule would replace a length of 3 horizontal corridors with a four way intersection, regardless of rotation.
```
3,3
R
=
...
ccc
...
=
.C.
c5c
.C.
```

### Tile Descriptions
The characters in the rule files are mapped to tile types as follows:

#### Basic Tiles
```
c: Horizontal corridor
C: Vertical corridor
r: Room
R: Room2 - renders the same as room but rules consider them distinct
~: Water
.: Empty space
```

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