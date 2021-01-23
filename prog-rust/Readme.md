# Приклади коду для _Programming Rust_

Цей репозиторій містить частину коду з книги “Programming Rust”, Jim Blandy та Jason Orendorff.

Приклади знаходяться під ліцензією MIT.

## Частина "Мандельброт"

Різні файли показують різні стратегії реалізації:

*   Файл `mandelbrot-single-threaded` базова версія програми. Вся робота робиться в головному потоці виконання.

*   Файл `mandelbrot-bands` splits the plotting area up into eight bands, and assigns one thread
    to each.  This often makes poor use of the threads, because some
    bands take significantly longer than others to complete: once a fast
    thread completes its band, its CPU goes idle while its less
    fortunate brethren are still hard at work.

*   Файл `mandelbrot-task-queue` gets an almost perfect linear speedup from its threads. It splits
    the plotting area up into many more bands, and then has threads draw
    bands from a common pool until the pool is empty. When a thread
    finishes one band, it goes back for more work. Since the bands still
    take different amounts of time to render, the problem cited above
    still occurs, but on a much smaller scale.

*   Файл `mandelbrot-lockfree` uses Rust's atomic types to implement a lock-free iterator type, and
    uses that to dole out bands from the pool instead of a
    mutex-protected count. On Linux, this is no faster than the
    mutex-based version, which isn't too surprising: on Linux, locking
    and unlocking an uncontended mutex *is* simply a pair of atomic
    operations.

*   `mandelbrot-rayon` uses the Rayon library instead of Crossbeam. Rayon provides a
    *parallel iterator* API that makes our code much simpler.  It looks
    a lot like Rust code that uses plain old iterators.



## Частина 9: Характеристики

- Файл `queue` визначає тип `Queue`, який представляє чергу значень `char`.

- Файл `generic-queue` містить код для універсального типу `Queue`.

## Частина 10: Перелічування та шаблони

- Файл `binary-tree` містить тип `BinaryTree`, що відображені в секціях “Generic Enums” та “Populating a Binary Tree”.

## Частина 12: Оператори перевантаження

- Файл `complex` містить тип `Complex`.

- Файл `interval` містить тип `Interval`, що реалізує характеристику `std::cmp::PartialOrd`.

## Частина 14: Замикання

- The 'basic-router' directory holds the `BasicRouter` type used as an example
  in the “Callbacks” section.

## Частина 15: Ітератори

- The `binary-tree` directory holds the implementation of the `Iterator` trait
  for the `BinaryTree` type originally defined in the “Enums and Patterns”
  chapter.

## Частина 17: Рядки і текст

- Файл `complex` включає реалізацію `std::fmt::Display` для форматування характеристики складного числового типу, показаного в секції “Formatting Your Own Types”.

## Частина 18: Ввід та ввивід

- The `grep` directory holds the simple grep-like program shown in the section
  “Reading Lines”.

- The `copy` directory holds the program for copying directory trees from the
  section “Reading Directories”, including the additions shown in the next
  section, “Platform-Specific Features”.

- The `echo-server` directory holds the simple network service shown in the
  “Networking” section.

- The `http-get` directory holds the command-line program that uses the
  `reqwest` crate to carry out an HTTP request.

## Частина 19: Конкуренція

- The search engine used as a running example throughout the book has its own
  repository, at `https://github.com/ProgrammingRust/fingertips`.

- The Mandelbrot set plotter discussed in the section “Revisiting the Mandelbrot
  Set” also has its own repository, at `https://github.com/ProgrammingRust/mandelbrot`.
  The repository includes several branches exploring different implementations;
  see the repository's [README.md][mandel-readme] file for details.

## Частина 20: Макроси

- The `json-macro` directory holds the definition of the `json!` macro built in
  the section “The json! Macro”.
