const std = @import("std");

fn parse_line(input: []const u8) !u32 {
    var sum: u32 = 0;
    var i: usize = 0;
    while (i < input.len) : (i += 1) {
        if (std.fmt.charToDigit(input[i], 10)) |digit| {
            sum += 10 * digit;
            break;
        } else |_| {
            continue;
        }
    }
    if (i == input.len) {
        return error.InvalidData;
    }
    i = input.len - 1;
    while (i >= 0) : (i -= 1) {
        if (std.fmt.charToDigit(input[i], 10)) |digit| {
            sum += digit;
            break;
        } else |_| {
            continue;
        }
    }
    return sum;
}
fn parse_line2(input: []const u8) !u32 {
    var sum: u32 = 0;
    var cur_idx: usize = input.len;
    var digit: u32 = 0;
    const words = [_][]const u8{ "one", "two", "three", "four", "five", "six", "seven", "eight", "nine" };
    var i: usize = 0;
    while (i < words.len) : (i += 1) {
        const maybe_idx = std.mem.indexOf(u8, input, words[i]);
        if (maybe_idx) |idx| {
            if (idx < cur_idx) {
                cur_idx = idx;
                digit = @as(u32, @intCast(i)) + 1;
            }
        }
    }
    if (std.mem.indexOfAny(u8, input, "123456789")) |idx| {
        if (idx < cur_idx) {
            cur_idx = idx;
            digit = try std.fmt.charToDigit(input[idx], 10);
        }
    }
    if (cur_idx == input.len) {
        return error.InvalidData;
    }
    sum += 10 * digit;
    cur_idx = 0;
    i = 0;
    while (i < words.len) : (i += 1) {
        const maybe_idx = std.mem.lastIndexOf(u8, input, words[i]);
        if (maybe_idx) |idx| {
            if (idx > cur_idx) {
                cur_idx = idx;
                digit = @as(u32, @intCast(i)) + 1;
            }
        }
    }
    if (std.mem.lastIndexOfAny(u8, input, "123456789")) |idx| {
        if (idx > cur_idx) {
            cur_idx = idx;
            digit = try std.fmt.charToDigit(input[idx], 10);
        }
    }
    sum += digit;
    return sum;
}

pub fn main() !void {
    const file = try std.fs.cwd().openFile("input.txt", .{ .mode = .read_only });
    defer file.close();

    var buf_reader = std.io.bufferedReader(file.reader());
    var in_stream = buf_reader.reader();

    var buffer: [1024]u8 = undefined;
    var sum1: u32 = 0;
    var sum2: u32 = 0;
    const stdout = std.io.getStdOut().writer();
    while (try in_stream.readUntilDelimiterOrEof(&buffer, '\n')) |line| {
        sum1 += try parse_line(line);
        sum2 += try parse_line2(line);
    }
    try stdout.print("Stage 1: {}\n", .{sum1});
    try stdout.print("Stage 2: {}\n", .{sum2});
}
