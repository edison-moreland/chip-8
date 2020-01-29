// Generic logic for single threaded chip-8 timers

// Relevant description of Chip-8's timers from Matt Mikolay:
//
// The CHIP-8 interpreter provides two programmable timers, the delay timer and 
// the sound timer. 
//
// Similar to a data register, a timer can be loaded with an unsigned 8-bit 
// value; the instructions FX15 and FX18 are used to set the values of the delay
// timer and sound timer respectively. When a timer is set to a non-zero value, 
// it will count down at a rate of sixty hertz until zero is reached. 

export class Timer {
    period: number
    last_time: number

    constructor(period: number) {
        console.log(period);
        this.period = period; // Miliseconds between cycles
        this.last_time = Date.now();
    }
    
    ticksPassed(): number {
        // Calculate the number of ticks since last called
        const now = Date.now();
        const between_time = now - this.last_time;
        
        if (between_time < this.period) {
            return 0;
        }
        
        const mod_between = (between_time%this.period);
        this.last_time = now + mod_between;
        return (between_time - mod_between) / this.period;
    }
}