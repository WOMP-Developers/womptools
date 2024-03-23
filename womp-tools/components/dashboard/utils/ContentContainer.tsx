
export default async function ContentContainer({ title, children }: { title: string, children?: React.ReactNode }) {
    return (
        <div className="flex flex-col flex-1 gap-2">
            <div className="px-2">
                { title }
            </div>
            <div>
                <div className="flex flex-row h-72 grow rounded-md bg-gray-950/20 backdrop-blur-2xl ring-1 ring-black/50 shadow-lg shadow-gray-950/60 p-2">
                    { children }
                </div>
            </div>
        </div>
    )
}
