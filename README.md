# General Idea

Terminal Task List for easier access.

Language: `Rust`

Resurces: [
  `diesel`,
  `postgres OR sqllite3`
];

# Design

## Detailed view

-----------------------------------------------------
|Created at: 12.04.2021, 12:00                      |
|Until: 13.04.2021, 12:45                           |
-----------------------------------------------------
|Title: Finish Mechanics Test                       |  
|Description(Optional):                             |  
| Lorem Ipsum is simply dummy text of the printing  |
|and typesetting industry. Lorem Ipsum has been the |
|industry's standard dummy text ever since the 1500s|
|when an unknown printer took a galley of type and  |
|scrambled it to make a type specimen book.         |
|It has survived not only five centuries,           |
|but also the leap into electronic typesetting,     |
|remaining essentially unchanged.                   |
-----------------------------------------------------

## List View
-----------------------------------------------------
|1. Task 1 ==> Due: 13.03.2021                      |
|2. Task 2 ==> Due: 13.03.2021                      |
|3. Task 3 ==> Due: 13.03.2021                      |
|4. Task 4 ==> Due: 13.03.2021                      |
|5. Task 5 ==> Due: 13.03.2021                      |
|6. Task 6 ==> Due: 13.03.2021                      |
|7. Task 7 ==> Due: 13.03.2021                      |
|8. Task 8 ==> Due: 13.03.2021                      |
|9. Task 9 ==> Due: 13.03.2021                      |
-----------------------------------------------------

# How to

1. Make an `API` to interact with the DATABASE
2. Establish commands for the `CLI`

3. Implement the `CLI`

# CLI Design

1. Adding the task

```shell
task add 

=========

Title:
<++>
Description:
<++>
Due date:
<++>
```
2. Showing the tasks

```shell
task list || tasks

==========

[<Tasks design>](##-Detailed-view)
```
3. Details on specific task

```shell
task <title>

==========

[<Details design>](##-List-View)
```
