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
