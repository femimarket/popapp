"use client"

import React, { useState } from "react"
import { ArrowDown, ArrowUp, ChevronDown } from "lucide-react"

import { Button } from "@/components/ui/button"
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select"
import { Switch } from "@/components/ui/switch"
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs"
import TradingViewWidget from "@/app/go/chart";

// Mock data for the chart
const data = [
  { time: "09:00", price: 100 },
  { time: "10:00", price: 120 },
  { time: "11:00", price: 110 },
  { time: "12:00", price: 130 },
  { time: "13:00", price: 125 },
  { time: "14:00", price: 140 },
  { time: "15:00", price: 135 },
]

export default function Component() {
  const [isBinaryOption, setIsBinaryOption] = useState(false)

  return (
      <div className="flex flex-col h-screen bg-background text-foreground">
        <header className="flex justify-between items-center p-4 border-b">
          <h1 className="text-2xl font-bold">Proof of Profit Protocol</h1>
          <div className="flex items-center gap-4">
            <span className="font-semibold">Simulated Capital:</span>
            <span className="text-xl font-bold">$100,000</span>
          </div>
        </header>
        <div className="flex flex-1 overflow-hidden">
          <Card className="w-80 m-4 flex flex-col justify-end">
            <CardHeader>
              <CardTitle>Trading Panel</CardTitle>
              <CardDescription>Execute your trades here</CardDescription>
            </CardHeader>
            <CardContent className="overflow-auto">
              <div className="mt-auto">
                <Tabs defaultValue="market" className="w-full">
                  <TabsList className="grid w-full grid-cols-2">
                    <TabsTrigger value="market">Market</TabsTrigger>
                    <TabsTrigger value="limit">Limit</TabsTrigger>
                  </TabsList>
                  <TabsContent value="market">
                    <div className="space-y-4">
                      <div className="flex items-center space-x-2">
                        <Label htmlFor="binary-option">Binary Option</Label>
                        <Switch
                            id="binary-option"
                            checked={isBinaryOption}
                            onCheckedChange={setIsBinaryOption}
                        />
                      </div>
                      {isBinaryOption ? (
                          <>
                            <div className="space-y-2">
                              <Label htmlFor="direction">Direction</Label>
                              <Select>
                                <SelectTrigger id="direction">
                                  <SelectValue placeholder="Select direction" />
                                </SelectTrigger>
                                <SelectContent>
                                  <SelectItem value="up">
                                    <div className="flex items-center">
                                      <ArrowUp className="mr-2 h-4 w-4 text-green-500" />
                                      Up
                                    </div>
                                  </SelectItem>
                                  <SelectItem value="down">
                                    <div className="flex items-center">
                                      <ArrowDown className="mr-2 h-4 w-4 text-red-500" />
                                      Down
                                    </div>
                                  </SelectItem>
                                </SelectContent>
                              </Select>
                            </div>
                            <div className="space-y-2">
                              <Label htmlFor="expiry">Expiry</Label>
                              <Select>
                                <SelectTrigger id="expiry">
                                  <SelectValue placeholder="Select expiry" />
                                </SelectTrigger>
                                <SelectContent>
                                  <SelectItem value="5m">5 minutes</SelectItem>
                                  <SelectItem value="15m">15 minutes</SelectItem>
                                  <SelectItem value="1h">1 hour</SelectItem>
                                </SelectContent>
                              </Select>
                            </div>
                          </>
                      ) : (
                          <div className="space-y-2">
                            <Label htmlFor="side">Side</Label>
                            <Select>
                              <SelectTrigger id="side">
                                <SelectValue placeholder="Select side" />
                              </SelectTrigger>
                              <SelectContent>
                                <SelectItem value="buy">Buy</SelectItem>
                                <SelectItem value="sell">Sell</SelectItem>
                              </SelectContent>
                            </Select>
                          </div>
                      )}
                      <div className="space-y-2">
                        <Label htmlFor="amount">Amount</Label>
                        <Input id="amount" placeholder="Enter amount" type="number" />
                      </div>
                      <Button className="w-full">
                        {isBinaryOption ? "Place Binary Option" : "Place Market Order"}
                      </Button>
                    </div>
                  </TabsContent>
                  <TabsContent value="limit">
                    <div className="space-y-4">
                      <div className="space-y-2">
                        <Label htmlFor="limit-side">Side</Label>
                        <Select>
                          <SelectTrigger id="limit-side">
                            <SelectValue placeholder="Select side" />
                          </SelectTrigger>
                          <SelectContent>
                            <SelectItem value="buy">Buy</SelectItem>
                            <SelectItem value="sell">Sell</SelectItem>
                          </SelectContent>
                        </Select>
                      </div>
                      <div className="space-y-2">
                        <Label htmlFor="limit-price">Limit Price</Label>
                        <Input id="limit-price" placeholder="Enter limit price" type="number" />
                      </div>
                      <div className="space-y-2">
                        <Label htmlFor="limit-amount">Amount</Label>
                        <Input id="limit-amount" placeholder="Enter amount" type="number" />
                      </div>
                      <Button className="w-full">Place Limit Order</Button>
                    </div>
                  </TabsContent>
                </Tabs>
              </div>
            </CardContent>
          </Card>
          <div className="flex-1 p-4 overflow-hidden">
            <TradingViewWidget asset={"PYTH:BTCUSD"}/>

          </div>
        </div>
      </div>
  )
}