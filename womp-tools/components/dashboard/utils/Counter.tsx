"use client"

import { useEffect, useState } from "react";

export default function Counter({ value }: { value: number }) {
    const [displayedValue, setDisplayedValue] = useState(0);

    useEffect(() => {
        const animation_duration = 1000; // 1 second
        const animation_interval = Math.floor(1000 / 30);
        const animation_steps = (animation_duration / animation_interval);

        const initial_value = displayedValue;
        const target_value = value;

        let animation_step = 1;

        const interval = setInterval(() => {
            let alpha = animation_step / animation_steps;
            setDisplayedValue(initial_value * (1 - alpha) + target_value * alpha);

            animation_step += 1;

            if (animation_step > animation_steps) {
                setDisplayedValue(target_value);
                clearInterval(interval);
            }
        }, animation_interval);

        return () => {
            clearInterval(interval);
        }
    }, [value])

    return (
        <>
            {`${displayedValue.toLocaleString('en-US', { maximumFractionDigits: 0 })}`}
        </>
    )
}