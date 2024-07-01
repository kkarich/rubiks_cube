const colorClassMap = {
    Red: 'bg-red-500',
    Blue: 'bg-blue-500',
    Green: 'bg-green-500',
    Yellow: 'bg-yellow-500',
    White: 'bg-white-500',
    Orange: 'bg-orange-500',
};

export default function Face({ data }) {
    return (
        <div className="space-y-1">
            {data.map((row, ri) => {
                return (
                    <div key={`row-${ri}`} className="flex space-x-1">
                        {row.map((col, ci) => {
                            const bgColorClass = colorClassMap[col];
                            return <div key={`${ri}-${ci}-${col}`} className={`${bgColorClass} h-8 w-8 rounded border-2 border-slate-500`}></div>;
                        })}
                    </div>
                );
            })}
        </div>
    );
}
