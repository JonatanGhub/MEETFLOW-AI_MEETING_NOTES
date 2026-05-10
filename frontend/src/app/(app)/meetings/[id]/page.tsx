import MeetingDetailPage from "./MeetingDetailClient";

// Required by Next.js output: "export" for dynamic routes.
// The actual meeting ID is resolved client-side via useParams() in MeetingDetailClient.
export function generateStaticParams() {
  return [];
}

export default function Page() {
  return <MeetingDetailPage />;
}
