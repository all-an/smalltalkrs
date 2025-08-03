//! Base Object trait and core object system for Smalltalk
//! 
//! In Smalltalk, everything is an object. This module defines the fundamental
//! Object trait that all Smalltalk objects must implement, providing the basic
//! operations for object identity, equality, and string representation.

use std::any::Any;
use std::fmt;

/// Unique identifier for each object instance in the Smalltalk system
/// 
/// Every object has a unique ID that remains constant throughout its lifetime.
/// This enables object identity comparisons (==) distinct from equality (=).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ObjectId(u64);

impl ObjectId {
    /// Creates a new unique ObjectId
    /// 
    /// # Returns
    /// A new ObjectId with a unique identifier
    /// 
    /// # Examples
    /// ```
    /// use smalltalkrs::core::ObjectId;
    /// let id1 = ObjectId::new();
    /// let id2 = ObjectId::new();
    /// assert_ne!(id1, id2);
    /// ```
    pub fn new() -> Self {
        use std::sync::atomic::{AtomicU64, Ordering};
        static COUNTER: AtomicU64 = AtomicU64::new(1);
        ObjectId(COUNTER.fetch_add(1, Ordering::Relaxed))
    }
    
    /// Returns the raw ID value
    /// 
    /// # Returns
    /// The underlying u64 identifier
    pub fn value(&self) -> u64 {
        self.0
    }
}

impl fmt::Display for ObjectId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{}", self.0)
    }
}

/// Base trait that all Smalltalk objects must implement
/// 
/// This trait provides the fundamental operations that every object in the
/// Smalltalk system supports: identity, equality, and string representation.
/// It corresponds to the root Object class in traditional Smalltalk systems.
pub trait SmalltalkObject: Any + fmt::Debug + Send + Sync {
    
    /// Returns the unique identity of this object
    /// 
    /// Object identity is used for the `==` operator in Smalltalk,
    /// which tests if two object references point to the same object.
    /// 
    /// # Returns
    /// The ObjectId uniquely identifying this object instance
    fn object_id(&self) -> ObjectId;
    
    /// Tests object identity (equivalent to Smalltalk's `==`)
    /// 
    /// Returns true if this object and the other object are the same instance.
    /// This is different from equality (`equals`), which tests logical equivalence.
    /// 
    /// # Arguments
    /// * `other` - Another object to compare with
    /// 
    /// # Returns
    /// True if both objects have the same identity
    /// 
    /// # Examples
    /// ```
    /// // Two different objects with same content are not identical
    /// let obj1 = SomeObject::new("hello");
    /// let obj2 = SomeObject::new("hello");
    /// assert!(!obj1.is_identical(&obj2));
    /// 
    /// // Same object reference is identical to itself
    /// assert!(obj1.is_identical(&obj1));
    /// ```
    fn is_identical(&self, other: &dyn SmalltalkObject) -> bool {
        self.object_id() == other.object_id()
    }
    
    /// Tests object equality (equivalent to Smalltalk's `=`)
    /// 
    /// Returns true if this object is logically equal to the other object.
    /// Default implementation uses identity, but subclasses should override
    /// to provide meaningful equality semantics.
    /// 
    /// # Arguments
    /// * `other` - Another object to compare with
    /// 
    /// # Returns
    /// True if both objects are logically equal
    fn equals(&self, other: &dyn SmalltalkObject) -> bool {
        self.is_identical(other)
    }
    
    /// Returns a string representation of this object
    /// 
    /// Equivalent to Smalltalk's `printString` method.
    /// Default implementation returns the class name and object ID.
    /// 
    /// # Returns
    /// A string describing this object
    fn to_smalltalk_string(&self) -> String {
        format!("a {} {}", std::any::type_name::<Self>(), self.object_id())
    }
}

// Extension to enable downcasting for trait objects
impl dyn SmalltalkObject {
    /// Enables downcasting to concrete types for type checking
    /// 
    /// This method allows checking if an object is of a specific type
    /// and safely casting it to that type for further operations.
    /// 
    /// # Returns
    /// A reference to self as Any trait object
    pub fn as_any(&self) -> &dyn Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    /// Test implementation of SmalltalkObject for testing purposes
    #[derive(Debug)]
    struct TestObject {
        id: ObjectId,
        value: i32,
    }
    
    impl TestObject {
        fn new(value: i32) -> Self {
            Self {
                id: ObjectId::new(),
                value,
            }
        }
    }
    
    impl SmalltalkObject for TestObject {
        fn object_id(&self) -> ObjectId {
            self.id
        }
        
        fn equals(&self, other: &dyn SmalltalkObject) -> bool {
            if let Some(other_test) = other.as_any().downcast_ref::<TestObject>() {
                self.value == other_test.value
            } else {
                false
            }
        }
        
        fn to_smalltalk_string(&self) -> String {
            format!("TestObject({}) {}", self.value, self.id)
        }
    }
    
    
    #[test]
    fn test_object_id_uniqueness() {
        let id1 = ObjectId::new();
        let id2 = ObjectId::new();
        let id3 = ObjectId::new();
        
        assert_ne!(id1, id2);
        assert_ne!(id2, id3);
        assert_ne!(id1, id3);
    }
    
    #[test]
    fn test_object_id_display() {
        let id = ObjectId::new();
        let display = format!("{}", id);
        assert!(display.starts_with('#'));
        assert_eq!(display, format!("#{}", id.value()));
    }
    
    #[test]
    fn test_object_identity() {
        let obj1 = TestObject::new(42);
        let obj2 = TestObject::new(42);
        
        // Same object is identical to itself
        assert!(obj1.is_identical(&obj1));
        
        // Different objects are not identical, even with same value
        assert!(!obj1.is_identical(&obj2));
        assert!(!obj2.is_identical(&obj1));
    }
    
    #[test]
    fn test_object_equality() {
        let obj1 = TestObject::new(42);
        let obj2 = TestObject::new(42);
        let obj3 = TestObject::new(24);
        
        // Objects with same value are equal
        assert!(obj1.equals(&obj2));
        assert!(obj2.equals(&obj1));
        
        // Objects with different values are not equal
        assert!(!obj1.equals(&obj3));
        assert!(!obj3.equals(&obj1));
        
        // Object is equal to itself
        assert!(obj1.equals(&obj1));
    }
    
    #[test]
    fn test_identity_vs_equality() {
        let obj1 = TestObject::new(42);
        let obj2 = TestObject::new(42);
        
        // Equal but not identical
        assert!(obj1.equals(&obj2));
        assert!(!obj1.is_identical(&obj2));
    }
    
    #[test]
    fn test_to_smalltalk_string() {
        let obj = TestObject::new(42);
        let string_repr = obj.to_smalltalk_string();
        
        assert!(string_repr.contains("TestObject(42)"));
        assert!(string_repr.contains(&format!("{}", obj.object_id())));
    }
    
    #[test]
    fn test_object_id_value() {
        let id = ObjectId::new();
        assert!(id.value() > 0);
        
        let another_id = ObjectId::new();
        assert!(another_id.value() > id.value());
    }
    
    #[test]
    fn test_default_equals_implementation() {
        // Test the default equals implementation on line 102-103
        #[derive(Debug)]
        struct DefaultObject {
            id: ObjectId,
        }
        
        impl DefaultObject {
            fn new() -> Self {
                Self {
                    id: ObjectId::new(),
                }
            }
        }
        
        impl SmalltalkObject for DefaultObject {
            fn object_id(&self) -> ObjectId {
                self.id
            }
            // Uses default equals implementation (should use is_identical)
        }
        
        let obj1 = DefaultObject::new();
        let obj2 = DefaultObject::new();
        
        // Default equals implementation uses identity
        assert!(obj1.equals(&obj1)); // Same object
        assert!(!obj1.equals(&obj2)); // Different objects
    }
    
    #[test]
    fn test_default_to_smalltalk_string() {
        // Test the default to_smalltalk_string implementation on line 113-114
        #[derive(Debug)]
        struct DefaultStringObject {
            id: ObjectId,
        }
        
        impl DefaultStringObject {
            fn new() -> Self {
                Self {
                    id: ObjectId::new(),
                }
            }
        }
        
        impl SmalltalkObject for DefaultStringObject {
            fn object_id(&self) -> ObjectId {
                self.id
            }
            // Uses default to_smalltalk_string implementation
        }
        
        let obj = DefaultStringObject::new();
        let string_repr = obj.to_smalltalk_string();
        
        // Should contain "a" prefix, type name, and object ID
        assert!(string_repr.starts_with("a "));
        assert!(string_repr.contains("DefaultStringObject"));
        assert!(string_repr.contains(&format!("{}", obj.object_id())));
    }
}