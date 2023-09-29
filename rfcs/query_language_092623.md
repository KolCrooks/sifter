# SIFTER query language

## Standard Operators
These operators are available for all non vector types.
| operator | description | example |
| --- | --- | --- |
| == | equal to | `id == 1` |
| != | not equal to | `id != 1` |
| > | greater than | `id > 1` |
| < | less than | `id < 1` |
| >= | greater than or equal to | `id >= 1` |
| <= | less than or equal to | `id <= 1` |

## Vector Operators:
These operators are available for only vector types.
| operator | description | example |
| --- | --- | --- |
| within | distance from | `vector within 0.2` |
| topk | top k nearest neighbors | `vector topk 10` |
| == | equal to. | `vector == $1` (currently only support for binary vectors) |

## Logical Operators:
| operator | description | example |
| --- | --- | --- |
| && | and | `id == 1 && vector within 0.2` |
| \|\| | or | `id == 1 \|\| vector within 0.2` |
| ! | not | `!(id == 1)` |
