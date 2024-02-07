# 👩🏼‍⚖️ Arbiter

Arbiter is a simple resource scheduler.

## Prompt

- Develop a basic scheduler service in Rust.
    - Handle resource reservation requests for specific time frames.
    - Here a ‘resource’ is represented simply as an integer smaller than the total capacity.
    - Implement scheduling and error handling for non-viable requests.
    - Total available capacity increases and decreases on a schedule that you know in advance.
- Select an appropriate data store.
    - Decide how to represent the data. Be prepared to explain your thinking about the data store and the representation you chose.
    - Create interfaces for the service to interact with the data store.
- Your service should have this function for interacting with the schedule:
    - `reserve(start_time, end_time, amount)`
    - Times are in unix epoch format. Implement appropriate errors for impossible requests.
- Write tests for expected behavior and edge cases.
- Here’s two example total capacity schedules, you’d use them while instantiating the data store:
    - Schedule 1
        - `{1707165008, 1708374608, 64}`
        - `{1708374608, 1710793808, 96}`
        - `{1710793808, 1711398608, 32}`
        - `{1711398608, 1713213008, 128}`
    - Schedule 2
        - `{1707165008, 1707769808, 50}`
        - `{1707769808, 1708979408, 80}`
        - `{1708979408, 1709584208, 40}`
        - `{1709584208, 1712003408, 100}`
        - `{1712003408, 1712608208, 20}`
        - `{1712608208, 1714422608, 60}`

## 🖥️ Installation

todo: write installation section in `README.md`

## 🖱️ Usage

todo: write usage section in `README.md`

## 🛠️ Contributing

todo: write contributing section in `README.md`

## 📐 Design Decisions

### Limitations

Reservation requests must start and end within the provided schedule. While the resource may be available outside of this timeframe, that exceeds Arbiter's purview. Resource capacity outside of the schedule is assumed to be zero.

### Plan

- [x] logging
    - ~~[ ] initialize in `prelude.rs`~~
- [x] RESTful API with some tests
    - [ ] param checking
        - [ ] ? `start_time` and `end_time` are unix seconds
            - add test
            - throw informative error
        - [x] random parameters that we didn't ask for (ex. `"emojis": "lol"`)
            - add test
            - throw informative error
    - [x] POST `reserve(start_time, end_time, capacity_amount, user_id)`
        - original: `reserve(start_time, end_time, amount)`
        - add `user_id` for BI/marketing teams benefit
- [x] work out logic behind REST calls
    - test client tests
        - expected behavior
            - other stuff
                - [ ] ~~edge case: request is larger than total capacity~~
                    - > need to find in-situ b/c total cluster capacity isn't set.
                    - from prompt
                        - Here a ‘resource’ is represented simply as an integer smaller than the total capacity.
            - [x] ... **within** a timeframe fence
                - capacity **is** available -> allocate (add to db); return success
                    - start time
                        - now
                            - allocate (add to db), return reservation complete and started. UID: 4242
                        - not-now
                            - allocate (add to db), return reservation complete. UID: 4242
                - capacity **not** available -> return sorry
            - [x] ... **across** timeframe fences (need fx for combining inter-timeframe queries: "Create interfaces for the service to interact with the data store")
                - capacity **is** available -> allocate (add to db); return success
                - capacity **not** available -> return sorry
        - edge cases
                - [x] outside of schedule timeframe
                    - assume the worst: no capacity available
                        - default to zero
                        - error message: while resource may be available for the period you gave, it's outside of Arbiter's purview. Please choose a timeframe between {schedule_max} and {schedule_min}
                - [x] `start_time` before `end_time` and vice versa
                    - add test
                    - throw informative error
                    - deets
                        - `start_time` after `end_time`
                        - `end_time` after `start_time`
                - `start_time` and `end_time` are valid unix epoch examples
                    - ex.
                        - too many digits
                        - too few digits
                    - prompt source
                        - "Times are in unix epoch format. Implement appropriate errors for impossible requests."
                - ~~? `amount` exceeds total capacity of cluster at zero utilization?~~
- [x] wire up evaluator and RESTful API
- missing
    - hmmm
        - ? disallow negative period start or end (never going to reserve to 1970)
        - start times before now()
            - allow historical?
        - messy args to REST API: str instead of int
    - **Times are in unix epoch format. Implement appropriate errors for impossible requests.**
        - [ ] ? convert to unixtime object ASAP instead of `int`
        - [ ] ? `start_time` and `end_time` are unix seconds
            - add test
            - throw informative error
- [ ] Specific scheduling errors and handling
    - **Implement scheduling and error handling for non-viable requests.**
- [ ] migrate ~~file~~ struct to SQL DB backing
    - **Select an appropriate data store.**
        - **Decide how to represent the data. Be prepared to explain your thinking about the data store and the representation you chose.**
        - **Create interfaces for the service to interact with the data store.**
    - tables
        - user requests
            - user_id, start, end, denied/allowed (bool)
        - ? separate or combined?
            - active reservations
                - `{1707165008, 1708374608, 64}`
                - start, end, capacity
            - cluster capacity
                - `{1707165008, 1708374608, 64}`
                - start, end, capacity
- [ ] option for `now` in `start_time`
    - **? consider different outcomes for `start_time`**
        - starts immediately vs starts a week from now
            - now: account for spinup time?
            - ? Assume that everything reserved at least 30 mins ahead of time is already spun up?

### Future: nice-to-haves

- user ID tracking
    - BI folks: what should we include in the next datacenter that we build?
    - marketing folks: what's selling
    - SRE dashboard: is something busted in a weird way
- ? allocation edge cases?
    - ensure 15% "float" capacity for "just-wanna-try-it" folks
- Swagger spec docs for RESTful API
- add test for RESTful API initialization
- enhancement suggestions
    - "negotiator"
        - suggestions
            - next timeframe that capacity is available
            - less capacity during the same timeframe
        - polynomial "sliders" (y=mx+b)
            - x: time
            - y: capacity
        - audience
            - API users 
            - downsteam frontends
        - ? beginning of walled garden compute market?
- Add testable `Examples` to fx docstrings

### Known Unknowns

- ~~total capacity == total capacity of hardware?~~
    - total cap == total hardware cap
- ~~interval: live or batch?~~
    - live
- ~~optimization priority?~~
    - **resource utilization**
    - user latency
        - having to wait forever might get frustrating
            - could set aside a small amount for "just-try-it-out" users
    - reliability (resource failure fault tolerance)
        - what if schedule changes?
            - ex.
                - assume two datacenters: A100s and H100s in different locales
                - timeline
                    - customer buys A100 cluster
                    - A100 cluster floods
                    - scheduler needs to be able to reassign to H100 center
                        - will lose some money b/c lower $/hr
                        - won't lose all of the money
                        - won't lose reputation reliability
- ~~applicable to multiple types of resources?~~
    - just one
- ~~BI and marketing people want to query data?~~
    - free to add `user_id` to function request

### Assumptions

- provided schedule will never change or fail
- okay to lock up if someone's already making an allocation

### Architecture

- RESTful API
    - Warp b/c simple and composable
        - also, big `seamonstar` fan
    - runner up: Rocket
        - simpler, but only recently out of nightly
    - runner up: Tide
        - too young
    - allows for concurrent requests
        - "lock" allocations if a user makes a request while another is being assessed
            - SQL db allows
- SQL DB
    - pros
        - keep the implementation simple
        - easy to reason about
        - easy to build on
        - SQLite might be easier at first
        - future: PosgreSQL
    - why not files?
        - more complex to reason about
        - eventual performance issues: aggregating every row will start to add up over time
        - notable pros include simplicity and no external dependencies
    - access pattern
        - ORM would be too much at this stage

## 🐭 Misc.

### 🔌 Compatibility

todo: write compatibility section in `README.md`

### 🙏🏻 Kudos

Readme format inspired by [Make a README](https://www.makeareadme.com) and [awesome-readme](https://github.com/matiassingers/awesome-readme/tree/master).

Changelog format inspired by [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## 🛟 Support

# 🪪 License

