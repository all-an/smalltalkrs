//! Boolean implementation for Smalltalk
//! 
//! In Smalltalk, True and False are singleton objects that represent boolean values.
//! Unlike most languages, boolean logic is implemented through message passing
//! to these objects rather than primitive operations.

use super::object::{ObjectId, SmalltalkObject};

/// Represents the singleton True object in Smalltalk
/// 
/// True is a singleton object that represents the boolean value true.
/// All conditional logic in Smalltalk involves sending messages to True or False.
#[derive(Debug, Clone)]
pub struct True {
    id: ObjectId,
}

impl True {
    /// Creates the singleton True object
    /// 
    /// In a full implementation, this would return a reference to a singleton,
    /// but for simplicity we create new instances here.
    /// 
    /// # Returns
    /// A new True object
    /// 
    /// # Examples
    /// ```
    /// use smalltalkrs::core::True;
    /// let true_obj = True::new();
    /// assert!(true_obj.is_true());
    /// ```
    pub fn new() -> Self {
        Self {
            id: ObjectId::new(),
        }
    }
    
    /// Returns the boolean value (always true)
    /// 
    /// # Returns
    /// Always returns true
    pub fn is_true(&self) -> bool {
        true
    }
    
    /// Logical AND operation (equivalent to Smalltalk's `and:`)
    /// 
    /// In Smalltalk, `true and: [expression]` evaluates the block only if
    /// the receiver is true. Since True is always true, this always
    /// evaluates the expression.
    /// 
    /// # Arguments
    /// * `other` - Another boolean value to AND with
    /// 
    /// # Returns
    /// The result of the AND operation
    /// 
    /// # Examples
    /// ```
    /// use smalltalkrs::core::{True, False};
    /// let true_obj = True::new();
    /// let false_obj = False::new();
    /// 
    /// assert!(true_obj.and(&true_obj));
    /// assert!(!true_obj.and(&false_obj));
    /// ```
    pub fn and(&self, other: &dyn BooleanObject) -> bool {
        other.is_true()
    }
    
    /// Logical OR operation (equivalent to Smalltalk's `or:`)
    /// 
    /// In Smalltalk, `true or: [expression]` short-circuits and returns true
    /// without evaluating the block. Since True is always true, this always
    /// returns true.
    /// 
    /// # Arguments
    /// * `_other` - Another boolean value (ignored since true OR anything is true)
    /// 
    /// # Returns
    /// Always returns true
    /// 
    /// # Examples
    /// ```
    /// use smalltalkrs::core::{True, False};
    /// let true_obj = True::new();
    /// let false_obj = False::new();
    /// 
    /// assert!(true_obj.or(&true_obj));
    /// assert!(true_obj.or(&false_obj));
    /// ```
    pub fn or(&self, _other: &dyn BooleanObject) -> bool {
        true
    }
    
    /// Logical NOT operation (equivalent to Smalltalk's `not`)
    /// 
    /// Returns the logical negation of this boolean value.
    /// 
    /// # Returns
    /// Always returns false (negation of true)
    /// 
    /// # Examples
    /// ```
    /// use smalltalkrs::core::True;
    /// let true_obj = True::new();
    /// assert!(!true_obj.not());
    /// ```
    pub fn not(&self) -> bool {
        false
    }
}

/// Represents the singleton False object in Smalltalk
/// 
/// False is a singleton object that represents the boolean value false.
/// All conditional logic in Smalltalk involves sending messages to True or False.
#[derive(Debug, Clone)]
pub struct False {
    id: ObjectId,
}

impl False {
    /// Creates the singleton False object
    /// 
    /// In a full implementation, this would return a reference to a singleton,
    /// but for simplicity we create new instances here.
    /// 
    /// # Returns
    /// A new False object
    /// 
    /// # Examples
    /// ```
    /// use smalltalkrs::core::False;
    /// let false_obj = False::new();
    /// assert!(!false_obj.is_true());
    /// ```
    pub fn new() -> Self {
        Self {
            id: ObjectId::new(),
        }
    }
    
    /// Returns the boolean value (always false)
    /// 
    /// # Returns
    /// Always returns false
    pub fn is_true(&self) -> bool {
        false
    }
    
    /// Logical AND operation (equivalent to Smalltalk's `and:`)
    /// 
    /// In Smalltalk, `false and: [expression]` short-circuits and returns false
    /// without evaluating the block. Since False is always false, this always
    /// returns false.
    /// 
    /// # Arguments
    /// * `_other` - Another boolean value (ignored since false AND anything is false)
    /// 
    /// # Returns
    /// Always returns false
    /// 
    /// # Examples
    /// ```
    /// use smalltalkrs::core::{True, False};
    /// let true_obj = True::new();
    /// let false_obj = False::new();
    /// 
    /// assert!(!false_obj.and(&true_obj));
    /// assert!(!false_obj.and(&false_obj));
    /// ```
    pub fn and(&self, _other: &dyn BooleanObject) -> bool {
        false
    }
    
    /// Logical OR operation (equivalent to Smalltalk's `or:`)
    /// 
    /// In Smalltalk, `false or: [expression]` evaluates the block only if
    /// the receiver is false. Since False is always false, this evaluates
    /// the expression.
    /// 
    /// # Arguments
    /// * `other` - Another boolean value to OR with
    /// 
    /// # Returns
    /// The result of the OR operation
    /// 
    /// # Examples
    /// ```
    /// use smalltalkrs::core::{True, False};
    /// let true_obj = True::new();
    /// let false_obj = False::new();
    /// 
    /// assert!(false_obj.or(&true_obj));
    /// assert!(!false_obj.or(&false_obj));
    /// ```
    pub fn or(&self, other: &dyn BooleanObject) -> bool {
        other.is_true()
    }
    
    /// Logical NOT operation (equivalent to Smalltalk's `not`)
    /// 
    /// Returns the logical negation of this boolean value.
    /// 
    /// # Returns
    /// Always returns true (negation of false)
    /// 
    /// # Examples
    /// ```
    /// use smalltalkrs::core::False;
    /// let false_obj = False::new();
    /// assert!(false_obj.not());
    /// ```
    pub fn not(&self) -> bool {
        true
    }
}

/// Common trait for boolean objects (True and False)
/// 
/// This trait provides a common interface for boolean operations
/// that can be performed on both True and False objects.
pub trait BooleanObject {
    /// Returns whether this object represents true
    /// 
    /// # Returns
    /// true for True objects, false for False objects
    fn is_true(&self) -> bool;
    
    /// Logical AND operation
    /// 
    /// # Arguments
    /// * `other` - Another boolean object to AND with
    /// 
    /// # Returns
    /// The result of the AND operation
    fn and(&self, other: &dyn BooleanObject) -> bool;
    
    /// Logical OR operation
    /// 
    /// # Arguments
    /// * `other` - Another boolean object to OR with
    /// 
    /// # Returns
    /// The result of the OR operation
    fn or(&self, other: &dyn BooleanObject) -> bool;
    
    /// Logical NOT operation
    /// 
    /// # Returns
    /// The logical negation of this boolean value
    fn not(&self) -> bool;
}

impl BooleanObject for True {
    fn is_true(&self) -> bool {
        self.is_true()
    }
    
    fn and(&self, other: &dyn BooleanObject) -> bool {
        self.and(other)
    }
    
    fn or(&self, other: &dyn BooleanObject) -> bool {
        self.or(other)
    }
    
    fn not(&self) -> bool {
        self.not()
    }
}

impl BooleanObject for False {
    fn is_true(&self) -> bool {
        self.is_true()
    }
    
    fn and(&self, other: &dyn BooleanObject) -> bool {
        self.and(other)
    }
    
    fn or(&self, other: &dyn BooleanObject) -> bool {
        self.or(other)
    }
    
    fn not(&self) -> bool {
        self.not()
    }
}

impl SmalltalkObject for True {
    fn object_id(&self) -> ObjectId {
        self.id
    }
    
    fn equals(&self, other: &dyn SmalltalkObject) -> bool {
        // True objects are equal if they're both True (singleton semantics)
        other.as_any().downcast_ref::<True>().is_some()
    }
    
    fn to_smalltalk_string(&self) -> String {
        "true".to_string()
    }
}

impl SmalltalkObject for False {
    fn object_id(&self) -> ObjectId {
        self.id
    }
    
    fn equals(&self, other: &dyn SmalltalkObject) -> bool {
        // False objects are equal if they're both False (singleton semantics)
        other.as_any().downcast_ref::<False>().is_some()
    }
    
    fn to_smalltalk_string(&self) -> String {
        "false".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_true_creation() {
        let true_obj = True::new();
        assert!(true_obj.is_true());
    }
    
    #[test]
    fn test_false_creation() {
        let false_obj = False::new();
        assert!(!false_obj.is_true());
    }
    
    #[test]
    fn test_true_and_operations() {
        let true_obj = True::new();
        let false_obj = False::new();
        
        // true AND true = true
        assert!(true_obj.and(&true_obj));
        
        // true AND false = false
        assert!(!true_obj.and(&false_obj));
    }
    
    #[test]
    fn test_false_and_operations() {
        let true_obj = True::new();
        let false_obj = False::new();
        
        // false AND true = false
        assert!(!false_obj.and(&true_obj));
        
        // false AND false = false
        assert!(!false_obj.and(&false_obj));
    }
    
    #[test]
    fn test_true_or_operations() {
        let true_obj = True::new();
        let false_obj = False::new();
        
        // true OR true = true
        assert!(true_obj.or(&true_obj));
        
        // true OR false = true
        assert!(true_obj.or(&false_obj));
    }
    
    #[test]
    fn test_false_or_operations() {
        let true_obj = True::new();
        let false_obj = False::new();
        
        // false OR true = true
        assert!(false_obj.or(&true_obj));
        
        // false OR false = false
        assert!(!false_obj.or(&false_obj));
    }
    
    #[test]
    fn test_not_operations() {
        let true_obj = True::new();
        let false_obj = False::new();
        
        // NOT true = false
        assert!(!true_obj.not());
        
        // NOT false = true
        assert!(false_obj.not());
    }
    
    #[test]
    fn test_boolean_object_trait() {
        let true_obj = True::new();
        let false_obj = False::new();
        
        // Test trait methods
        let true_ref: &dyn BooleanObject = &true_obj;
        let false_ref: &dyn BooleanObject = &false_obj;
        
        assert!(true_ref.is_true());
        assert!(!false_ref.is_true());
        
        assert!(true_ref.and(false_ref) == false);
        assert!(false_ref.or(true_ref) == true);
        
        assert!(!true_ref.not());
        assert!(false_ref.not());
    }
    
    #[test]
    fn test_truth_table_and() {
        let true_obj = True::new();
        let false_obj = False::new();
        
        // Complete AND truth table
        assert_eq!(true_obj.and(&true_obj), true);   // T && T = T
        assert_eq!(true_obj.and(&false_obj), false); // T && F = F
        assert_eq!(false_obj.and(&true_obj), false); // F && T = F
        assert_eq!(false_obj.and(&false_obj), false); // F && F = F
    }
    
    #[test]
    fn test_truth_table_or() {
        let true_obj = True::new();
        let false_obj = False::new();
        
        // Complete OR truth table
        assert_eq!(true_obj.or(&true_obj), true);   // T || T = T
        assert_eq!(true_obj.or(&false_obj), true);  // T || F = T
        assert_eq!(false_obj.or(&true_obj), true);  // F || T = T
        assert_eq!(false_obj.or(&false_obj), false); // F || F = F
    }
    
    #[test]
    fn test_smalltalk_object_implementation() {
        let true_obj = True::new();
        let false_obj = False::new();
        
        // Test object IDs are unique
        assert_ne!(true_obj.object_id(), false_obj.object_id());
        
        // Test string representations
        assert_eq!(true_obj.to_smalltalk_string(), "true");
        assert_eq!(false_obj.to_smalltalk_string(), "false");
    }
    
    #[test]
    fn test_true_equality() {
        let true1 = True::new();
        let true2 = True::new();
        let false_obj = False::new();
        
        // All True objects should be equal (singleton semantics)
        assert!(true1.equals(&true2));
        assert!(true2.equals(&true1));
        
        // True should not equal False
        assert!(!true1.equals(&false_obj));
        
        // Self equality
        assert!(true1.equals(&true1));
    }
    
    #[test]
    fn test_false_equality() {
        let false1 = False::new();
        let false2 = False::new();
        let true_obj = True::new();
        
        // All False objects should be equal (singleton semantics)
        assert!(false1.equals(&false2));
        assert!(false2.equals(&false1));
        
        // False should not equal True
        assert!(!false1.equals(&true_obj));
        
        // Self equality
        assert!(false1.equals(&false1));
    }
    
    #[test]
    fn test_boolean_identity_vs_equality() {
        let true1 = True::new();
        let true2 = True::new();
        
        // Different True objects are equal but not identical
        assert!(true1.equals(&true2));
        assert!(!true1.is_identical(&true2));
        
        // Same object is identical to itself
        assert!(true1.is_identical(&true1));
    }
    
    #[test]
    fn test_equals_with_non_boolean() {
        // Test equality with non-boolean objects
        use super::super::small_integer::SmallInteger;
        
        let true_obj = True::new();
        let false_obj = False::new();
        let number = SmallInteger::new(42);
        
        // Booleans should not equal non-boolean objects
        assert!(!true_obj.equals(&number));
        assert!(!false_obj.equals(&number));
    }
    
    #[test]
    fn test_logic_laws() {
        let true_obj = True::new();
        let false_obj = False::new();
        
        // Law of excluded middle: P OR NOT P = true
        assert!(true_obj.or(&False::new()) || true_obj.not());
        assert!(false_obj.or(&True::new()) || false_obj.not());
        
        // Law of non-contradiction: NOT (P AND NOT P) = true
        assert!(!(true_obj.and(&False::new()) && true_obj.not()));
        assert!(!(false_obj.and(&True::new()) && false_obj.not()));
        
        // De Morgan's laws would require more complex setup
        // but basic cases can be tested
        assert_eq!(!true_obj.and(&false_obj), true_obj.not() || false_obj.not());
    }
    
    #[test]
    fn test_short_circuit_semantics() {
        let true_obj = True::new();
        let false_obj = False::new();
        
        // True OR anything should be true (simulating short-circuit)
        assert!(true_obj.or(&true_obj));
        assert!(true_obj.or(&false_obj));
        
        // False AND anything should be false (simulating short-circuit)
        assert!(!false_obj.and(&true_obj));
        assert!(!false_obj.and(&false_obj));
    }
    
    #[test]
    fn test_boolean_trait_implementation_coverage() {
        // This test specifically covers the trait implementation methods
        // that delegate to struct methods (lines 266-267, 280-281)
        let true_obj = True::new();
        let false_obj = False::new();
        
        // Cast to trait references to test trait implementations
        let true_trait: &dyn BooleanObject = &true_obj;
        let false_trait: &dyn BooleanObject = &false_obj;
        
        // Test True::or through trait (covers lines 266-267)
        assert!(true_trait.or(true_trait));  // true OR true = true
        assert!(true_trait.or(false_trait)); // true OR false = true
        
        // Test False::and through trait (covers lines 280-281)
        assert!(!false_trait.and(true_trait));  // false AND true = false
        assert!(!false_trait.and(false_trait)); // false AND false = false
    }
}