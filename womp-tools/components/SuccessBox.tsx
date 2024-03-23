
export default function SuccessBox({ message }: { message: string }) {
    return (
        <div className="p-4 bg-black/60 text-green-500 rounded-md">
            {message}
        </div>
    )
}
