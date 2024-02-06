# 👩🏼‍⚖️ Arbiter

Arbiter is a simple resource scheduler.

## Prompt

- Develop a basic scheduler service in Rust.
    - Handle resource reservation requests for specific time frames.
    - Here a ‘resource’ is represented simply as an integer smaller than the total capacity.
    - Implement scheduling and error handling for non-viable requests.
    - Total available capacity increases and decreases on a schedule that you know in advance.
- Select an appropriate data store.
    - Decide how to represent the data.
    - Be prepared to explain your thinking about the data store and the representation you chose.
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

### Plan

- [ ] log to `/tmp`
- [ ] RESTful API with some tests
    - `reserve(start_time, end_time, amount)`
        - add `user_id` for BI/marketing teams benefit
- [ ] work out logic behind REST calls
    - test client tests
        - expected behavior
            - ? consider different outcomes for `start_time`
                - starts immediately vs starts a week from now
                    - now: account for spinup time?
                    - ? assume that everything reserved at least 30 mins ahead of time is already spun up?
            - ... **within** timeframe fences
                - capacity **is** available -> allocate (add to db); return success
                    - start time
                        - now
                            - allocate (add to db), return reservation complete and started. UID: 4242
                        - not-now
                            - allocate (add to db), return reservation complete. UID: 4242
                - capacity **not** available - return sorry
            - ... **across** timeframe fences (need fx for combining inter-timeframe queries: "Create interfaces for the service to interact with the data store")
                - capacity **is** available -> allocate (add to db); return success
                - capacity **not** available - return sorry
        - edge cases
            - input checking
                - `start_time` and `end_time` are in unix epoch format
                    - "Times are in unix epoch format. Implement appropriate errors for impossible requests."
                - `start_time` after `end_time`
                - `end_tiem` after `start_time`
                - ? `amount` exceeds total capacity of cluster at zero utilization?
            - ? allocation edge cases?
                - ensure 15% "float" capacity for "just-wanna-try-it" folks
- [ ] migrate file to SQL DB backing 

### Future: nice-to-haves

- can't-do-but suggestions
    - "negotiator" suggestions
        - next timeframe that capacity is available
        - less capacity during the same timeframe
- user ID tracking
    - BI folks: what should we include in the next datacenter that we build?
    - marketing folks: what's selling
    - SRE dashboard: is something busted in a weird way

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

