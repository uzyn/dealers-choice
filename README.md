# Dealer's Choice

Mixed game poker evaluation engine, potentially also game engine.

Note: Early development. Unstable API.

## Games

The following games are in consideration for the library:

| Variant                           | Limits     | Gameplay  | Evaluation | Low eval     | Pot    |
| --------------------------------- | ---------- | --------- | ---------- | ------------ | ------ |
| Hold'em                           | FL, PL, NL | Community | High       |              | Single |
| Omaha                             | FL, PL     | Community | High       |              | Single |
| Omaha Hi-Lo 8 or Better           | FL, PL     | Community | High-low   | 8 or better  | Double |
| 5-card Omaha                      | FL, PL     | Community | High       |              | Single |
| Big O                             | FL, PL     | Community | High-low   | 8 or better  | Double |
| Bomb pot                          | PL         | Community | High       |              | Double |
| Seven Card Stud                   | FL         | Stud      | High       |              | Single |
| Seven Card Stud Hi-Lo 8 or Better | FL         | Stud      | Lowball    | 8 or better  | Double |
| Seven Card Stud Hi-Lo Regular     | FL         | Stud      | Lowball    | No qualifier | Double |
| Razz                              | FL         | Stud      | Lowball    | A-5          | Single |
| 2-7 Razz                          | FL         | Stud      | Lowball    | 2-7          | Single |
| 5-Card Draw                       | FL, PL, NL | Draw      | High       |              | Single |
| A-5 Triple Draw                   | FL         | Draw      | Lowball    | A-5          | Single |
| 2-7 Single Draw                   | NL         | Draw      | Lowball    | 2-7          | Single |
| 2-7 Triple Draw                   | FL         | Draw      | Lowball    | 2-7          | Single |
| Badugi                            | FL         | Draw      | Lowball    | Badugi       | Single |
| Badacey                           | FL         | Draw      | Lowball    | Badugi + A-5 | Double |
| Badeucey                          | FL         | Draw      | Lowball    | Badugi + 2-7 | Double |

Not in consideration at the moment:

* Pineapple
* Chinese
* Flip & Go

## Reference

1. [WSOP Dealer's Choice event](https://www.wsop.com/pdfs/structuresheets/structure_5504_23147.pdf)
2. [Poker hand evaluation algorithm](https://github.com/HenryRLee/PokerHandEvaluator/blob/master/Documentation/Algorithm.md)
