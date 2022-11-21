```mermaid
classDiagram
  class App{
      +title: &'a str
      +should_quit: bool
      +tabs: TabsState<'a>
      +tables: StatefulList<&'a str>

      +on_up()
      +on_down()
      +on_right()
      +on_left()
      +on_key()
  }
```
