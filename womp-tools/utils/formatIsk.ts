
export default function formatIsk(value: number|undefined): string {
    if (!value) return "0";

    return value.toLocaleString('en-US', { maximumFractionDigits: 0 })
}