package clock

import "fmt"

const (
	HOURS_PER_DAY    = 24
	MINUTES_PER_HOUR = 60
	MINUTES_PER_DAY  = HOURS_PER_DAY * MINUTES_PER_HOUR
)

type Clock struct {
	hours   int
	minutes int
}

func New(h, m int) Clock {
	total_minutes := h*MINUTES_PER_HOUR + m
	if total_minutes < 0 {
		total_minutes = (total_minutes % MINUTES_PER_DAY) + MINUTES_PER_DAY
	}
	hours := (total_minutes / MINUTES_PER_HOUR) % HOURS_PER_DAY
	minutes := total_minutes % MINUTES_PER_HOUR
	return Clock{hours: hours, minutes: minutes}
}

func (c Clock) Add(m int) Clock {
	return New(c.hours, c.minutes+m)
}

func (c Clock) Subtract(m int) Clock {
	return c.Add(-m)
}

func (c Clock) String() string {
	return fmt.Sprintf("%02d:%02d", c.hours, c.minutes)
}
