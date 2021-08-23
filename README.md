# cli_graph
By Jamie Apps

A crate to allow generation of simple CLI graphs 

Current Features:
- Bar and Scatter Graph modes
- Coloured customisable graph ticks
- Customisable y-axis scales
- Multi-Line graphs for large x-axis ranges
- Variable height and width graphs


Example:

``` rust
let colours : Vec<Colour> = vec![Colour::Red, Colour::Green, Colour::Blue, Colour::Orange, Colour::Cyan];
let names   : Vec<String> = vec!["apples","oranges","bananas","grapes","mangos"].iter().map(|&s| s.to_owned() ).collect();
let values  : Vec<f64>    = vec![5.0,3.0,8.0,2.0,7.2];
let gd = GraphData::from((names, values, colours));
let gc = GraphConfig::new().max_height(11);
graph(gd, gc, GraphType::Bar).unwrap();
```

gives:
```bash
8   |               #                     
    |               #                     
6.5 |               #              #      
    |               #              #      
5   |#              #              #      
    |#              #              #      
3.5 |#              #              #      
    |#      #       #              #      
2   |#      #       #       #      #      
    ------------------------------------------
     apples oranges bananas grapes mangos
```
(Colours not reflected in markdown example)

**Note** : This crate is still quite young and it's API will likely change. If this is an issue, please constrain the version in the Cargo.toml.