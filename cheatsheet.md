## Cheat Sheet

###
Trait
  * Trait is like interface, other struct or object needs to implement it.
    
    `impl {traitName} for {structName}`

  * Trait can be pass as parameter like:

     ```
     pub fn notify(item: &impl {traitName}) {  // Take an item that implements traitName
         
     }
     ```

     Another way:

     ```
     pub fn notify<T: Summary>(item: &T) {  // Take an item that implements traitName
         
     }
     ```
     * multiple trait
     ```
     pub fn notify<T: Summary+Display>(item: &T) {  // Take an item that implements traitName
         
     }
     ```
