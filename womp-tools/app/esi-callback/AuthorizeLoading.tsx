import LoadingIndicator from "@/components/LoadingIndicator";

export default function AuthorizeLoading() {
    return (
        <>
            <h2 className={`mb-3 text-2xl font-semibold`}>
                Authorizing using ESI...
            </h2>

            <LoadingIndicator type="beat" />
        </>
    )
}
