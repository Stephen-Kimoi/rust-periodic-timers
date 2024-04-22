# RUST PERIODIC TIMERS
Internet computer offers a feature that allows you to trigger a task automatically within a certain specified interval. 

We use the ``ic_cdk_timers:set_timer_interval()`` to create a periodic timer with the specified interval 

```
ic_cdk_timers::set_timer_interval(interval, || { // Creates a new periodic timer with the specified interval and a closure to call 
        COUNTER.fetch_add(1, Ordering::Relaxed); // Increases the global counter everytime the periodic task is triggered 
    }); 
```

### Deploying the canister locally: 
When deploying the canister, one needs to pass the ``--argument`` flag followed by the arguemt value serialized in candid format. 

If we want to set the interval of the periodic task to 1 second, we use the following command: 
```
dfx deploy my_timers_backend --argument "(1:nat64)"
```
