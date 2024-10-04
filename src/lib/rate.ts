export interface Rates {
    rates: {
        HUF: string
        USD: string
        EUR: string
    }
}

export function get_rates(): string[][] {
    let list: string[][] = []
    list.push(['huf', 'HUP', 'Magyar Pénz'])
    list.push(['usd', 'USD', 'Amerikai Dollár'])
    list.push(['eur', 'EUR', 'Euró'])
    return list
}
