import LoadingIndicator from "@/components/LoadingIndicator";

export default async function Loading() {
    return (
    <div className="flex grow justify-center p-12">
        <LoadingIndicator type={"beat"}  />
    </div>
    )
}