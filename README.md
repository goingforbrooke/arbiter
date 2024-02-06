# 👩🏼‍⚖️ Arbiter

Arbiter is a simple resource scheduler.

Prompt:

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
    - reserve(start_time, end_time, amount)
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

todo: write design decisions section in `README.md`

## 🐭 Misc.

### 🔌 Compatibility

todo: write compatibility section in `README.md`

### 🙏🏻 Kudos

Readme format inspired by [Make a README](https://www.makeareadme.com) and [awesome-readme](https://github.com/matiassingers/awesome-readme/tree/master).

Changelog format inspired by [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## 🛟 Support

# 🪪 License

