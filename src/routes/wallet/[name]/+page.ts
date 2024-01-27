
export function load({ params }: { params: { name: string }}) {	
    let name: string = params.name
    return {
        name,
        balance: 200
    }
}
