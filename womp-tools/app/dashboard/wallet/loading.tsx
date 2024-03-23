import LoadingIndicator from "@/components/LoadingIndicator";

export default async function Loading() {
    return (
        <div className="flex flex-col grow justify-center items-center">
            <LoadingIndicator type={'beat'} />
        </div>
    )
}