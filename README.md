# slope_diversity
A thread simulation in Rust showing how different groups in Computer Science can grow

Computer Science Diversity Simulation.
This program creates 3 groups of threads.

* Low CS Group -> Meant to represent people without much CS privilege. (0)

* Medium CS Group -> Meant to represent people with medium CS privilege. (20)

* High CS Group -> Meant to represent  people with high CS priviledge. (50)

These groups start off at different point values to begin.
Each group will be able to study(), a blocking function which has a probability to 
increase an individuals' CS points. 
The goal of a thread is to finish 3 CS classes (simulated to be CS106A, CS106B, and CS107, courses at Stanford.)
They will send a message to the CS Department thread with the following: 

* Identifier for the thread (LOW_GROUP_1 or MED_GROUP_5)
    
* Class they're currently taking 
    
* Amount of CS Points to their name. 
    
The department will inform the thread if they can move on to another class, of if they've failed and require 
another try. Each thread is given 3 tries (for all the classes). If they run out of tries, they 'drop out' the major.
After all threads in group finish.

The CS Department Thread keeps track of each student thread's progress, and outputs this HashMap into "table.txt" as JSON. I've included a Python notebook that analyzes the result for a given set. 
     
What statistics will we find? Run the executable and get a simulation for yourself :) 
Organizations to support CS Diversity: 

http://www.code2040.org/

https://girlswhocode.com/
