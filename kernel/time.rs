const std = @import("std");

// This isn't the library routine, it is only used in the kernel.
// as such, we don't care about years<1970 etc, but assume everything
// is ok. Similarly, TZ etc is happily ignored. We just do everything
// as easily as possible. Let's find something public for the library
// routines (although I think minix times is public).
//
// PS. I hate whoever though up the year 1970 - couldn't they have gotten
// a leap-year instead? I also hate Gregorius, pope or no. I'm grumpy.

const MINUTE = 60;
const HOUR = 60 * MINUTE;
const DAY = 24 * HOUR;
const YEAR = 365 * DAY;

// interestingly, we assume leap-years
const month = [_]i32{
    0,
    DAY * (31),
    DAY * (31 + 29),
    DAY * (31 + 29 + 31),
    DAY * (31 + 29 + 31 + 30),
    DAY * (31 + 29 + 31 + 30 + 31),
    DAY * (31 + 29 + 31 + 30 + 31 + 30),
    DAY * (31 + 29 + 31 + 30 + 31 + 30 + 31),
    DAY * (31 + 29 + 31 + 30 + 31 + 30 + 31 + 31),
    DAY * (31 + 29 + 31 + 30 + 31 + 30 + 31 + 31 + 30),
    DAY * (31 + 29 + 31 + 30 + 31 + 30 + 31 + 31 + 30 + 31),
    DAY * (31 + 29 + 31 + 30 + 31 + 30 + 31 + 31 + 30 + 31 + 30),
};

const Tm = struct {
    tm_sec: i32,
    tm_min: i32,
    tm_hour: i32,
    tm_mday: i32,
    tm_mon: i32,
    tm_year: i32,
    // outros campos podem ser adicionados se necessário
};

fn kernel_mktime(tm: *const Tm) i64 {
    var res: i64 = 0;
    const year = tm.tm_year - 70;

    // magic offsets (y+1) needed to get leapyears right.
    res = YEAR * @as(i64, year) + DAY * @as(i64, (year + 1) / 4);
    res += month[@intCast(usize, tm.tm_mon)];
    
    // and (y+2) here. If it wasn't a leap-year, we have to adjust
    if (tm.tm_mon > 1 and @mod(year + 2, 4) != 0) {
        res -= DAY;
    }
    
    res += DAY * @as(i64, tm.tm_mday - 1);
    res += HOUR * @as(i64, tm.tm_hour);
    res += MINUTE * @as(i64, tm.tm_min);
    res += tm.tm_sec;
    
    return res;
}

// Exemplo de uso (opcional)
pub fn main() void {
    var tm = Tm{
        .tm_sec = 0,
        .tm_min = 0,
        .tm_hour = 0,
        .tm_mday = 1,
        .tm_mon = 0,
        .tm_year = 70, // 1970
    };
    
    const timestamp = kernel_mktime(&tm);
    std.debug.print("Timestamp: {d}\n", .{timestamp});
}
