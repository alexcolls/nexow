import { defineStore } from 'pinia'

export const useEngineStore = defineStore('engine', {
  state: () => ({
    running: false,
    metrics: [] as any[],
    bars: [] as any[],
    ws: null as WebSocket | null,
  }),
  actions: {
    connectWs(url: string) {
      if (this.ws) {
        this.ws.close()
      }
      this.ws = new WebSocket(url)
      this.ws.onmessage = (ev) => {
        const evt = JSON.parse(ev.data)
        if (evt.Bar) {
          this.bars.push(evt.Bar)
        }
        if (evt.Metrics) {
          this.metrics.push(evt.Metrics)
        }
      }
      this.ws.onopen = () => {
        console.log('WebSocket connected')
        this.running = true
      }
      this.ws.onclose = () => {
        console.log('WebSocket disconnected')
        this.running = false
      }
      this.ws.onerror = (error) => {
        console.error('WebSocket error:', error)
      }
    },
    disconnect() {
      if (this.ws) {
        this.ws.close()
        this.ws = null
      }
      this.running = false
    }
  },
})
