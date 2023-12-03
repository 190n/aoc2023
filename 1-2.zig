const std = @import("std");

// fn WindowReader(comptime len: usize) type {
//     return struct {
//         const Self = @This();

//         buffer: [len]u8,
//         reader: std.io.BufferedReader(4096, std.io.AnyReader),
//         buffer_fullness: usize = 0,
// 		at_eof: bool = false,

//         pub fn init(reader: std.io.AnyReader) Self {
//             return .{
//                 .buffer = undefined,
//                 .reader = std.io.bufferedReader(reader),
//             };
//         }

//         pub fn next(self: *Self) ?[]const u8 {
//             if (self.buffer_fullness < self.buffer.len) {

// 			}
//         }
//     };
// }

pub fn main() !void {
    var buf: [5]u8 = undefined;
    var slice: []u8 = buf[0..1];

    const stdin = std.io.getStdIn();
    var buffered_reader = std.io.bufferedReader(stdin.reader());
    const reader = buffered_reader.reader();

    var first_digit: ?u8 = null;
    var last_digit: u8 = 0;
    var sum: u64 = 0;
    var hit_eof = false;

    read_loop: while (true) {
        if (try reader.readAll(slice[slice.len - 1 ..]) == 0) {
            hit_eof = true;
            if (slice.len <= 1) break :read_loop;
            slice = slice[0 .. slice.len - 1];
        }

        const current_digit = blk: {
            if ('0' <= slice[0] and slice[0] <= '9') {
                break :blk slice[0] - '0';
            }

            inline for (.{
                .{ 1, "one" },
                .{ 2, "two" },
                .{ 3, "three" },
                .{ 4, "four" },
                .{ 5, "five" },
                .{ 6, "six" },
                .{ 7, "seven" },
                .{ 8, "eight" },
                .{ 9, "nine" },
            }) |tup| {
                const digit, const string = tup;
                if (std.mem.startsWith(u8, slice, string)) {
                    break :blk digit;
                }
            }
            break :blk null;
        };

        if (current_digit) |digit| {
            if (first_digit == null) first_digit = digit;
            last_digit = digit;
        } else if (slice[0] == '\n') {
            sum += 10 * first_digit.? + last_digit;
            first_digit = null;
        }

        if (slice.len < buf.len and !hit_eof) {
            slice = buf[0 .. slice.len + 1];
        } else {
            std.mem.copyForwards(u8, buf[0 .. buf.len - 1], buf[1..]);
        }
    }

    std.debug.print("{}\n", .{sum});
}
