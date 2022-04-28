#Dungeon Grammars
##A grid grammar based dungeon generator

##Rule syntax
###Options
All rules start with a line showing their dimensions in Columns,Rows format
Rules which should have rotations generated should have an "R" options

Example:
3,3,R produces a rule which matches with a 3x3 space with rotations
2,3 produces a rule which matches with a 2x3 space without rotations

###Patterns and Results
The first grid described is the pattern which will be matched
Grids are separated by lines with equals signs
Further grids are possible results which will replace the pattern grid 
Result grids are picked randomly with equal weight

The following rule would replace a length of 3 horizontal corridors with a four way intersection.

3,3,R
...
ccc
...
=
.C.
c5c
.C.

###Tile Descriptions
The following characters describe the following tile types:

####Basic Tiles
c: Horizontal corridor
C: Vertical corridor
r: Room
.: Empty space

####Intersection Tiles
These are based on the numpad

789             ╔╦╗
456 --maps-to-> ╠╬╣
123             ╚╩╝

###Wildcard Tiles
The ? character has different behaviour depending on whether its in the pattern or the rule.

If they are in the pattern, they ignore the tile type and act as a wildcard.

If the are in the result, whatever was there originally is not replaced.