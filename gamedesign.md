# Game Design Document

This document contains the sketch and idea of the game. The document iself is no garantuee that the game implements the mechanics in that way. It is more a notepad for my thoughts.

## Base idea

The game consists of dedicated levels. Each level presents the player with a set of tracks, trains, a start tile and a goal tile. The level is solved when the entity at the start tile (Sam) reaches the end goal.

The tiles are hexagons.

Sam can use trains to reach the end tile. Sam is transfered from train to train when a train collides with another. Every collision of a train accelerates or stops the train (when both are hit at the head).

The player can configure the start timer of each train to plan and start the excution of the plan. If Sam doesn't reach the goal, the level is reset and the player has to replan.

The execution of the train is deterministic:

1.  Each train moves by its given speed (curves reduces the speed) one game tick (could be a second)
2.  The collisions are detected and handled
3.  Trains give a portion of their speed to the hit train. The following schema shows the bonus given to the hit train:
    ```
           STOP
            |
            v
            _
    -1  ->/   \ <- -1
    +1  ->\   / <- +1
            ¯
            ^
            |
           +4
    ```
4.  Loop until no train moves or the end goal is reached.

The level has two different goals - reach the Goal tile fastest with the most collisions. Each collisions brings more and more points. The player should be motivated to find a fast way and maximize train collisions. It should be beneficial to add more train collisions even when they do not transfer Sam faster.

Trains do not loose speed unless they are going into a curve or hitting another train.
