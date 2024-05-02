> [!WARNING]
> Cargo will give you some errors because I use `rand` crate
> Which causes problems with trait `Distribution<T>` since I
> use generic typization for some users which might use `u8`
> or `i16` within the neurons and weights so your advice would
> be very helpful! Create an Issue where you would give me
> your advice! Thank You Very Much!

# MindMatrix AI
This is a library for AI and other neural networks stuff. It can do:
* machine learning and huge data processing capabilities
* saving AI into a **.ai** file and using it as a library
* it can insert the AI model from file into your project right at
    the compilation, and your AI will be used by app without including
    it separately, which is good for microcontrollers
