<template>
  <v-container fluid class="d-flex align-center justify-center">
    <v-row justify="center">
      <v-col cols="12" sm="8" md="6">
        <v-card class="pa-4">
          <!-- Form section -->
          <v-form @submit.prevent="submitUrls" class="mb-4">
            <v-text-field v-model="urls" label="Enter search term" required></v-text-field>
            <v-select v-model="numberOfPages" :items="pageOptions" label="How many pages"></v-select>
            <v-checkbox v-model="sequential" label="Sequential"></v-checkbox>
            <v-btn type="submit" color="primary" :disabled="loading">
              <template v-if="loading">
                <span>Loading...</span>
              </template>
              <template v-else>
                Scrape
              </template>
            </v-btn>
          </v-form>

          <!-- Loading indicator -->
          <template v-if="loading">
            <v-progress-linear color="primary" :model-value="loadingProgress"></v-progress-linear>
          </template>

          <!-- Divider and result section -->
          <v-divider v-if="scrapeResult" class="mt-4 mb-4"></v-divider>
          
          <v-container v-if="scrapeResult">
            <v-row>
              <v-col cols="12">
                <p>API call took: {{ apiCallDuration }} ms</p>
              </v-col>
            </v-row>
            <v-row>
              <v-col cols="12">
                <v-select v-model="sortBy" :items="sortOptions" label="Sort by" @change="sortResults"></v-select>
              </v-col>
            </v-row>
            <v-row>
              <v-col v-for="(item, index) in sortedItems" :key="index" cols="12" class="mb-3">
                <v-card>
                  <v-card-title>{{ item.name }}</v-card-title>
                  <v-card-subtitle>{{ item.price }}</v-card-subtitle>
                  <v-card-text>Reviews: {{ item.review_count }}, Sales: {{ item.sale_count }}</v-card-text>
                  <v-card-actions>
                    <a :href="item.link" target="_blank">
                      <v-btn text>View Details</v-btn>
                    </a>
                  </v-card-actions>
                </v-card>
              </v-col>
            </v-row>
          </v-container>
        </v-card>
      </v-col>
    </v-row>
  </v-container>
</template>

<script>
import axios from 'axios';

export default {
  data() {
    return {
      urls: '',
      scrapeResult: null,
      numberOfPages: 1,
      pageOptions: [1, 2, 3, 4, 5],
      sortBy: null,
      sortOptions: [
        "None",
        "Reviews Count",
        "Sales Count"
      ],
      loading: false,
      loadingProgress: 0,
      apiCallDuration: 0,
      sequential: false,
    };
  },
  computed: {
    sortedItems() {
      if (!this.scrapeResult || !this.sortBy || this.sortBy === 'None') {
        return this.scrapeResult ? this.scrapeResult.items : [];
      }

      return [...this.scrapeResult.items].sort((a, b) => {
        if (this.sortBy === 'Reviews Count') {
          return b.review_count - a.review_count;
        } else if (this.sortBy === 'Sales Count') {
          return b.sale_count - a.sale_count;
        }
        return 0;
      });
    }
  },
  methods: {
    async submitUrls() {
      try {
        this.loading = true; // Start loading

        const links = [];
        for (let page = 1; page <= this.numberOfPages; page++) {
          links.push(`https://www.lazada.co.th/catalog/?page=${page}&q=${encodeURIComponent(this.urls)}`);
        }

        // Simulate loading progress
        this.loadingProgress = 0;
        const totalProgressSteps = this.numberOfPages * 10;

        for (let step = 1; step <= totalProgressSteps; step++) {
          await this.delay(this.calculateLoadTime());
          this.loadingProgress = Math.floor((step / totalProgressSteps) * 100);
        }

        const startTime = performance.now(); // Capture start time
        const apiUrl = this.sequential ? 'http://localhost:3030/seq-scrape' : 'http://localhost:3030/scrape';
        const response = await axios.post(apiUrl, {
          urls: links
        }, {
          headers: {
            'Content-Type': 'application/json'
          }
        });
        const endTime = performance.now(); // Capture end time

        this.apiCallDuration = Math.round(endTime - startTime); // Calculate and store API call duration
        this.scrapeResult = response.data;
      } catch (error) {
        console.error('Error scraping URLs:', error);
        alert("Error scraping URLs. Please try again later.");
      } finally {
        this.loading = false; // Stop loading
        this.loadingProgress = 0; // Reset progress
      }
    },
    sortResults() {
    },
    calculateLoadTime() {
      const avgLoadTime = 400; 
      const randomFactor = Math.random() * 2 - 1;
      return avgLoadTime + avgLoadTime * randomFactor;
    },
    delay(ms) {
      return new Promise(resolve => setTimeout(resolve, ms));
    }
  },
};
</script>

<style scoped>
.v-card {
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
}
</style>
