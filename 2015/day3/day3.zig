const std = @import("std");
const Position = struct { x: i32, y: i32 };
const House = struct { location: Position, no_of_presents: u32 };

fn readFile(allocator: std.mem.Allocator, path: []const u8) ![]u8 {
    var file = try std.fs.cwd().openFile(path, .{});
    defer file.close();
    const file_info = try file.stat();
    const buffer = try allocator.alloc(u8, file_info.size);
    _ = try file.readAll(buffer);
    return buffer;
}

fn firstHalf(allocator: std.mem.Allocator, data: []u8) !u32 {
    var position: Position = Position{ .x = 0, .y = 0 };
    var house = std.AutoHashMap(Position, u16).init(allocator);

    _ = try house.put(position, 1);

    for (data) |char| {
        switch (char) {
            '>' => position.x += 1,
            '<' => position.x -= 1,
            '^' => position.y += 1,
            'v' => position.y -= 1,
            else => break,
        }

        if (house.getPtr(position)) |p| {
            _ = try house.put(position, p.* + 1);
        } else {
            _ = try house.put(position, 1);
        }
    }
    return house.count();
}

fn secondHalf(allocator: std.mem.Allocator, data: []u8) !u32 {
    var position_santa = Position{ .x = 0, .y = 0 };
    var position_robosanta = Position{ .x = 0, .y = 0 };

    var visited_positions = std.AutoHashMap(Position, void).init(allocator);
    defer visited_positions.deinit();

    try visited_positions.put(position_santa, {});

    var i: usize = 0;
    while (i < data.len) : (i += 2) {
        if (i < data.len) {
            switch (data[i]) {
                '>' => position_santa.x += 1,
                '<' => position_santa.x -= 1,
                '^' => position_santa.y += 1,
                'v' => position_santa.y -= 1,
                else => {},
            }
            _ = try visited_positions.put(position_santa, {});
        }

        if (i + 1 < data.len) {
            switch (data[i + 1]) {
                '>' => position_robosanta.x += 1,
                '<' => position_robosanta.x -= 1,
                '^' => position_robosanta.y += 1,
                'v' => position_robosanta.y -= 1,
                else => {},
            }
            _ = try visited_positions.put(position_robosanta, {});
        }
    }

    return visited_positions.count();
}

pub fn run() !void {
    var allocator: std.mem.Allocator = std.heap.page_allocator;
    const data: []u8 = try readFile(allocator, "2015/day3/day3.txt");
    defer allocator.free(data);
    var no_of_houses_visited_once = try firstHalf(allocator, data);
    std.debug.print("First Half (no_of_houses_visited_once): {}\n", .{no_of_houses_visited_once});
    no_of_houses_visited_once = try secondHalf(allocator, data);
    std.debug.print("Second Half (no_of_houses_visited_once): {}\n", .{no_of_houses_visited_once});
}
