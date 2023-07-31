-- The function get_name() should return a single string that is the name of the puzzle.
--
function get_name()
    return "delay"
end

-- The function get_description() should return an array of strings, where each string is
-- a line of description for the puzzle. Surrounding text with asterisks will cause it to 
-- be rendered in bold, something we use when mentioning a signal by name.
--
-- By using the syntax that we use in our puzzle descriptions, you can also create tables:
--
--       "| This is a table. | This is a second column. | This is a column with "
--       "|                  |                          | multiple line of text."
--       "----------------------------------------------------------------------"
--       "| This is a second |                          | More text here!       "
--       "| table row.       |                          |                       "
--
-- Puzzle descriptions are not automatically paginated, so you can use the string "<PAGE>"
-- to start a new page of text. If you don't it will overflow and make Zach sad.
--
function get_description()
    return { 
        "*in* is an XBus input.",
        "*out* is an XBUS output.",
        "When a value is received from the *input*, forward this value to the *output* after 5 steps.",
    }
end

-- The function get_board() allows you to specify an 18x7 "ASCII art" grid to customize
-- the layout of the board and the placement of the input and output terminals. Generally
-- speaking, inputs are placed on the left of boards, while outputs are placed on the right.
--
-- For empty space, use the '.' character. 
-- For buildable board space, use the '#' character. 
-- For an input or output terminal, use characters '0' through '9' no more than once each.
-- For the bottom-left corner of the radio, use the 'R' character.
-- For the bottom-left corners of dials, use characters 'A', 'B', and 'C'.
--
function get_board()
    return [[
        ..................
        ......########....
        ....#########1....
        ....##########....
        ....0#########....
        ....########......
        ..................
    ]]
end

-- The function get_data() is called both to establish information about the puzzle (such as what
-- the inputs and outputs are) and to generate the random test cases. Signal levels and XBus data
-- should change from call to call, but information like the names and types of terminals should
-- not change at all.
--
-- To create a standard input or output terminal, call create_terminal(). Valid terminal types are
-- TYPE_SIMPLE, TYPE_XBUS, and TYPE_XBUS_NONBLOCKING. Valid terminal directions are DIR_INPUT and 
-- DIR_OUTPUT. Valid data for a simple I/O signal is an array of integers, 0 - 100 inclusive. Valid
-- data for an XBus signal is an array of integer arrays, each with values -999 to 999 inclusive.
--
--       create_terminal(name, board_character, type, direction, data)
--
-- To create a radio (C2S-RF901), call create_radio(). You may only create one radio in each puzzle.
-- Since radios are XBus-only, the only valid data for data_rx and data_tx are arrays of integer arrays,
-- each with values -999 to 999 inclusive. You cannot customize the signal names for a radio.
--
-- By default the radio will be placed in the bottom-left corner of the screen. However, if you use an 
-- 'R' character in your board layout, the bottom-left corner of the radio will be placed there instead.
--
--       create_radio(data_rx, data_tx)
--
-- To create a dial (N4DL-1000), call create_dial(). You may create up to three dials in each puzzle.
-- The names of dials should be kept short, as there is not much room to display them visually. A valid
-- value is an integer between 0 and 99, inclusive.
--
-- By default dials will be placed in the bottom-left corner of the screen. However, if you use an
-- 'A', 'B', or 'C' character in your board layout, the bottom-left corners of the first, second, and
-- third dials will be placed there, respectively.
--
--       create_dial(name, value)
--
-- NOTE: To generate random values you should use math.random(). However, you SHOULD NOT seed
--       the random number generator with a new seed value, as that is how the game ensures that
--       the first test run is consistent for all users, and thus something that allows for the
--       comparison of cycle scores.
--
-- NOTE: Fun fact! Arrays in Lua are implemented as tables (dictionaries) with integer keys that
--       start at 1 by convention. Contrast this with nearly every other programming language, in
--       which arrays start with an index of 0. Because of this, the 60 "time slices" that make
--       up a test case are indexed from 1 to 60 inclusive.
--
function get_data()
    input = {}
    output = {}
	time = math.random(2, 3)

    while time <= 55 do

        -- Generate a random keypress and make a single-value XBus data packet with it:
        packet = {}
        num_inputs = math.random(1,4);
        i = 1

        while i <= num_inputs do
            key = math.random(-999, 999)
            packet[i] = key
            i = i + 1
        end

        input[time] = { packet }
        output[time+5] = { packet }

        -- Add a random delay between inputs:
        time = time + math.random(1, 5)
    end

    create_terminal("input", "0", TYPE_XBUS, DIR_INPUT, input)
    create_terminal("output", "1", TYPE_SIMPLE, DIR_OUTPUT, output)
end
