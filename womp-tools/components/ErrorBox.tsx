
export default function ErrorBox({ message }: { message: string }) {
    return (
        <div className="p-2 bg-black/60 text-red-500 rounded-md">
            {message}
        </div>
    )
}
