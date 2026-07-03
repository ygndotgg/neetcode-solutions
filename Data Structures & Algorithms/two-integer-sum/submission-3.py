class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        numbers = []
        for i , num in enumerate(nums):
            numbers.append([num,i])
        numbers.sort()
        i , j = 0, len(numbers) -1
        while i < j:
            curr = numbers[i][0] + numbers[j][0]
            if curr == target:
                return [min(numbers[i][1],numbers[j][1]),max(numbers[i][1],numbers[j][1])]
            elif curr < target:
                i +=1
            else:
                j-=1
        return []
        