
export default function TimeTrackingContent({ bgColor }: { bgColor: string }) {
    return (
        <div className={`p-6 rounded-lg ${bgColor}`}>
            <h1 className="text-2xl font-bold mb-4">勤怠</h1>
            <p>ここに勤怠の情報が表示されます。</p>
        </div>
    );
}
