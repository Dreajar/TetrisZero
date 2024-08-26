# TetrisZero
Welcome to TetrisZero, a project showcasing the capabilities of Rust as a systems programming language and its advantages over traditional languages like C++. This game was developed within two weeks as a testament to the rapid learning curve and powerful features of Rust.

# About the Game
This is a classic Tetris game with a twist – it supports multiple players in a networked environment. Players can join a lobby, compete against each other, and enjoy the classic gameplay with a modern twist.

# Features
* Multiplayer Support: Play with friends over a network.
* Asynchronous Operations: Utilizes Rust's powerful async features for non-blocking gameplay.
* Real-time Interaction: Responsive input handling for a smooth gaming experience.
* Rust Language Showcase: Demonstrates memory safety, concurrency, and performance optimizations.

# Why Rust Over C++?
Rust offers several improvements over C++, making it an ideal choice for system-level programming and game development:
* Memory Safety: Rust's ownership model eliminates data races and ensures safe concurrency.
* Performance: Zero-cost abstractions mean that Rust code can be as fast as C++.
* Modern Concurrency: Rust's built-in concurrency primitives are safer and easier to use than C++ threads.
* Error Handling: Rust's Result and Option types encourage robust error handling.

# How I Wrote the Project
* Extensive Documentation: Rust's comprehensive documentation was instrumental in accelerating the learning process. It provided clear, concise explanations and examples that facilitated understanding the language's nuances.
* Type Checking System: Rust's robust type system and compiler error messages were invaluable. They not only ensured type safety but also acted as a real-time guide, pointing out what was missing or incorrect in the code as I developed the game. 
* Modular Design: The game was architected using Rust's module system, which helped in organizing the code into distinct, manageable components such as board, game, player, and server.
* Asynchronous Networking: By utilizing Rust's async features and the Tokio runtime, the game server was able to handle multiple players and input events efficiently without blocking operations.
* Test-Driven Development (TDD): Each module was developed with a focus on testing, using Rust's built-in testing framework. This approach ensured that components were reliable and interacted correctly with one another.
* Iterative Refinement: Leveraging Rust's strong compile-time checks allowed for rapid iteration and refinement of the codebase, quickly identifying and fixing issues during compilation rather than at runtime.

# Challenges Faced
While Rust has proven to be a powerful language for systems programming, it also presented some challenges, particularly in the area of machine learning integration:

Async Programming
* Initial Confusion: Asynchronous programming in Rust was initially confusing due to its different paradigms and concepts such as futures, tasks, and executors.
* Community Resources: Overcoming this hurdle was greatly facilitated by the wealth of community resources available, including blogs, discussions, videos, and Reddit posts. These resources provided practical insights and solutions to specific issues I encountered.
* Learning Curve: Through persistence and learning from the community, I gained a deeper understanding of async programming, which is now a fundamental part of my skill set.

Borrow Checker
* Adjusting to Borrow Checker: Coming from a C++ background, I initially struggled with Rust's borrow checker. It was a paradigm shift to think in terms of ownership, borrowing, and lifetimes rather than manual memory management.
* Overcomplicating with Boxes and Cells and RefCells: In an attempt to circumvent the borrow checker, I utilized Box, Cell, and RefCell, which led to a proliferation of bugs and a more complex codebase than necessary.
* Realization and Redesign: After spending considerable time wrestling with these issues, I realized that the root cause was an overcomplicated design. I took a step back and reevaluated my approach to design, focusing on simplicity and embracing Rust's ownership model.

Learning and Adaptation
* Over Complexity: I learned the importance of simplicity in design, especially when working with Rust. Instead of trying to force-fit C++-style solutions, I started to think in terms of Rust's idioms and patterns.
* Embracing Ownership: By fully embracing Rust's ownership and borrowing principles, I was able to write code that was not only more in line with Rust's philosophy but also safer and easier to understand.
* Iterative Refinement: I adopted an iterative approach to design, frequently revisiting and refining my code to ensure it adhered to Rust's principles without sacrificing functionality.
* Improved Code Quality: The redesign led to a cleaner, more maintainable codebase that was less prone to bugs and more idiomatic to Rust.
* Deepened Understanding: This process deepened my understanding of Rust's core concepts and improved my ability to leverage them effectively in my code, even in other languages.

Machine Learning
This project is called TetrisZero because there was supposed to be a singleplayer mode where you'd get to play against an AI. However,
* ML Ecosystem: Rust's machine learning ecosystem is not as mature as that of Python or C++. This made it difficult to implement advanced AI for the single-player mode against bots.
* Foreign Function Interface (FFI): Interfacing Rust with other languages, particularly for machine learning models, was a complex task. The FFI with Python, while possible, was not straightforward and required careful management to avoid performance overheads. Similarly, interfacing with C or C++ for leveraging existing ML libraries resulted in large file sizes, which are often binary executables or libraries that need to be dynamically linked.

Tokio
* Differences from Standard Library: Using tokio::net::TcpStream for network communication was challenging at first because of its differences from the standard library's std::net::TcpStream. The async nature of tokio's networking required a different approach to handling connections and data transfer.
* Adapting to Async I/O: I had to adapt my thinking and coding style to work with async I/O operations, which involved understanding concepts like async read and write operations, handling async streams, and managing tasks and their lifecycles.
* Tokio's Learning Resources: The tokio documentation and community examples were invaluable in helping me grasp the necessary concepts and implement a robust server capable of handling multiple players concurrently.

# Lessons Learned
* Language Interoperability: The project highlighted the importance of language interoperability when integrating specialized libraries or functionalities not yet available in Rust.
* Trade-offs in Tooling: The choice of programming language involves trade-offs, including the availability of third-party libraries and the ease of integration with other systems.

# Future Considerations
Community and Ecosystem Growth: As the Rust community grows, the ecosystem is expected to mature, potentially offering better support for machine learning in the future. For now, a hybrid approach using Rust for performance-critical parts and other languages for areas like ML might be necessary. 

# License
This project is licensed under the MIT License.
