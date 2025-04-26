# Ultra-Clean Global Provider System for Mix

You're absolutely right - passing context around is cumbersome. Let's create a truly global, macro-based system that feels effortless:

```rust
// ====== Provider Definition ======

// Define a basic state provider
define_provider!(COUNTER: StateProvider<i32> = 0);

// Define a computed provider 
define_provider!(DOUBLED: Provider<i32> = |_| watch!(COUNTER) * 2);

// Define a named provider (multiple of same type)
define_provider!(USER_NAME: StateProvider<String, "username"> = "John");
define_provider!(APP_TITLE: StateProvider<String, "app_title"> = "My App");

// Define an async provider
define_provider!(USER: FutureProvider<User> = async {
    api_client::fetch_user().await
});

// Define a parameterized provider
define_provider!(USER_DETAILS: FamilyProvider<UserId, UserDetails> = |id| async move {
    api_client::fetch_user_details(id).await
});

// ====== Using Providers ======

struct CounterView;

impl Widget for CounterView {
    fn build(&self) -> impl Widget {
        // Get state with global watch macro - no context needed!
        let count = watch!(COUNTER);
        let doubled = watch!(DOUBLED);
        let username = watch!(USER_NAME);
        
        Column::new()
            .children([
                // Use state directly
                Text::new(format!("Count: {}", count)),
                Text::new(format!("Doubled: {}", doubled)),
                Text::new(format!("Hello, {}", username)),
                
                // Update state with direct access
                Button::new("Increment")
                    .on_click(|_| {
                        // No context needed! Global access
                        modify!(COUNTER, |n| *n += 1);
                        // Alternative syntax for simple updates
                        update!(COUNTER += 1);
                    }),
                
                // Reset counter
                Button::new("Reset")
                    .on_click(|_| {
                        // Set state directly
                        set!(COUNTER, 0);
                    }),
                
                // Change username
                Button::new("Change Name")
                    .on_click(|_| {
                        set!(USER_NAME, "Alice");
                    }),
                
                // Using async data
                AsyncView!(USER, {
                    loading: || Text::new("Loading..."),
                    error: |e| Text::new(format!("Error: {}", e)),
                    data: |user| UserView::new(user),
                }),
                
                // Using family provider
                let user_id = watch!(SELECTED_USER_ID);
                AsyncView!(USER_DETAILS(user_id), {
                    loading: || Text::new("Loading details..."),
                    error: |e| Text::new(format!("Error: {}", e)),
                    data: |details| DetailsView::new(details),
                }),
            ])
    }
}
```

## Implementation Details

This system would be built using:

### 1. Global Provider Container

```rust
// Hidden implementation
thread_local! {
    static PROVIDER_CONTAINER: RefCell<ProviderContainer> = RefCell::new(ProviderContainer::new());
}

// Track current widget being built
thread_local! {
    static CURRENT_COMPONENT: RefCell<Option<ComponentId>> = RefCell::new(None);
}
```

### 2. Smart Macros for Provider Definition

```rust
#[macro_export]
macro_rules! define_provider {
    // Standard provider
    ($name:ident: StateProvider<$type:ty> = $initial:expr) => {
        static $name: ProviderId = {
            let id = ProviderId::new::<$type>();
            PROVIDER_CONTAINER.with(|container| {
                container.borrow_mut().register_provider(
                    id, 
                    Box::new($initial), 
                    None
                );
            });
            id
        };
    };
    
    // Named provider (for multiple of same type)
    ($name:ident: StateProvider<$type:ty, $provider_name:literal> = $initial:expr) => {
        static $name: ProviderId = {
            let id = ProviderId::new_named::<$type>($provider_name);
            PROVIDER_CONTAINER.with(|container| {
                container.borrow_mut().register_provider(
                    id, 
                    Box::new($initial), 
                    Some($provider_name.to_string())
                );
            });
            id
        };
    };
    
    // Other variants for different provider types
    // ...
}
```

### 3. Access Macros

```rust
#[macro_export]
macro_rules! watch {
    ($provider:ident) => {
        {
            PROVIDER_CONTAINER.with(|container| {
                let mut container = container.borrow_mut();
                
                // Register dependency with current component
                CURRENT_COMPONENT.with(|current| {
                    if let Some(component_id) = *current.borrow() {
                        container.add_dependency(component_id, $provider);
                    }
                });
                
                // Get state (type-safe through static ProviderId)
                container.get_state($provider)
            })
        }
    };
    
    // Named provider access
    ($type:ty, $name:literal) => {
        {
            let provider_id = ProviderId::get_named::<$type>($name);
            // Same implementation as above
        }
    };
}

#[macro_export]
macro_rules! set {
    ($provider:ident, $value:expr) => {
        {
            PROVIDER_CONTAINER.with(|container| {
                let mut container = container.borrow_mut();
                container.set_state($provider, $value);
            })
        }
    };
}

#[macro_export]
macro_rules! update {
    ($provider:ident += $value:expr) => {
        modify!($provider, |val| *val += $value)
    };
    
    ($provider:ident -= $value:expr) => {
        modify!($provider, |val| *val -= $value)
    };
    
    // Other operator shortcuts
}

#[macro_export]
macro_rules! modify {
    ($provider:ident, $modifier:expr) => {
        {
            PROVIDER_CONTAINER.with(|container| {
                let mut container = container.borrow_mut();
                container.modify_state($provider, $modifier);
            })
        }
    };
}
```

### 4. Async View Helpers

```rust
#[macro_export]
macro_rules! AsyncView {
    ($provider:ident, { loading: $loading:expr, error: $error:expr, data: $data:expr }) => {
        {
            let state = watch!($provider);
            match state {
                AsyncState::Loading => $loading(),
                AsyncState::Error(e) => $error(e),
                AsyncState::Data(data) => $data(data),
            }
        }
    };
    
    // Variant for family providers
    ($provider:ident($param:expr), { loading: $loading:expr, error: $error:expr, data: $data:expr }) => {
        {
            let provider_id = $provider.with_param($param);
            let state = watch_raw!(provider_id);
            match state {
                AsyncState::Loading => $loading(),
                AsyncState::Error(e) => $error(e),
                AsyncState::Data(data) => $data(data),
            }
        }
    };
}
```

## Automatic Widget Rebuilding

The magic ingredient is how we manage widget rebuilding. In our widget rendering system:

```rust
// Inside widget rendering system (users never see this)
fn render_widget<W: Widget>(widget: &W) -> WidgetNode {
    // Set current component ID for watch! macros to use
    let component_id = get_or_create_component_id(widget);
    
    CURRENT_COMPONENT.with(|current| {
        *current.borrow_mut() = Some(component_id);
    });
    
    // Build the widget (all watch! calls will register dependencies)
    let node = widget.build();
    
    // Clear current component
    CURRENT_COMPONENT.with(|current| {
        *current.borrow_mut() = None;
    });
    
    node
}
```

This approach gives you:

1. **Global access** to state without passing context
2. **Type-safe providers** with minimal syntax
3. **Named providers** for multiple instances of the same type
4. **Clean, macro-based API** that hides all Rust complexity
5. **Automatic dependency tracking** for smart rebuilding

It combines the best of Flutter's Riverpod with Rust's safety in an incredibly clean API that would make UI development a joy.